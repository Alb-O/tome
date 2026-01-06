//! Code actions popup for displaying LSP code action suggestions.
//!
//! This module provides the [`CodeActionsPopup`] type which displays a list
//! of available code actions (quickfixes, refactors, source actions) at the
//! current cursor position.

use termina::event::{KeyCode, KeyEvent, MouseEventKind};
use xeno_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, Command, WorkspaceEdit,
};
use xeno_registry::themes::Theme;
use xeno_tui::buffer::Buffer;
use xeno_tui::layout::Rect;
use xeno_tui::style::{Color, Modifier, Style, Stylize};
use xeno_tui::symbols::border;
use xeno_tui::text::{Line, Span};
use xeno_tui::widgets::Widget;
use xeno_tui::Frame;

use super::{Popup, PopupAnchor, PopupEvent, PopupEventResult, SizeHints};

/// Maximum number of visible items in the code actions list.
const MAX_VISIBLE_ITEMS: usize = 12;

/// Maximum width for the code actions popup.
const MAX_WIDTH: u16 = 70;

/// Minimum width for the code actions popup.
const MIN_WIDTH: u16 = 25;

/// A popup for displaying LSP code actions.
///
/// CodeActionsPopup displays a list of available code actions (quickfixes, refactors, etc.)
/// from the language server. Actions are grouped by kind with clear visual distinction.
///
/// The popup is modal (captures all input) and anchored to the cursor.
pub struct CodeActionsPopup {
    /// All code actions received from the LSP.
    actions: Vec<CodeActionOrCommand>,
    /// Index of the currently selected action.
    selected: usize,
    /// Scroll offset for the visible window.
    scroll_offset: usize,
    /// Anchor position for the popup.
    anchor: PopupAnchor,
}

/// Result of selecting a code action.
#[derive(Debug, Clone)]
pub enum CodeActionResult {
    /// A workspace edit to apply.
    Edit(WorkspaceEdit),
    /// A command to execute on the server.
    Command(Command),
    /// Both an edit and a command.
    EditAndCommand(WorkspaceEdit, Command),
}

impl CodeActionsPopup {
    /// Creates a new code actions popup from an LSP CodeActionResponse.
    ///
    /// Returns `None` if the response is empty.
    pub fn from_response(actions: Vec<CodeActionOrCommand>) -> Option<Self> {
        if actions.is_empty() {
            return None;
        }

        // Sort actions: quickfixes first, then refactors, then source actions
        let mut sorted_actions = actions;
        sorted_actions.sort_by(|a, b| {
            let kind_a = Self::action_kind_priority(a);
            let kind_b = Self::action_kind_priority(b);
            kind_a.cmp(&kind_b)
        });

        Some(Self {
            actions: sorted_actions,
            selected: 0,
            scroll_offset: 0,
            anchor: PopupAnchor::cursor_below(),
        })
    }

    /// Creates a new code actions popup with explicit actions.
    pub fn new(actions: Vec<CodeActionOrCommand>) -> Option<Self> {
        Self::from_response(actions)
    }

    /// Returns the priority for sorting actions by kind.
    fn action_kind_priority(action: &CodeActionOrCommand) -> u8 {
        match action {
            CodeActionOrCommand::CodeAction(ca) => match &ca.kind {
                Some(kind) if kind.as_str().starts_with("quickfix") => 0,
                Some(kind) if kind.as_str().starts_with("refactor") => 1,
                Some(kind) if kind.as_str().starts_with("source") => 2,
                _ => 3,
            },
            CodeActionOrCommand::Command(_) => 4,
        }
    }

    /// Returns whether there are any actions to display.
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Returns the number of actions.
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Selects the next action in the list.
    pub fn select_next(&mut self) {
        if self.actions.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.actions.len();
        self.ensure_visible();
    }

    /// Selects the previous action in the list.
    pub fn select_prev(&mut self) {
        if self.actions.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.actions.len() - 1;
        } else {
            self.selected -= 1;
        }
        self.ensure_visible();
    }

    /// Ensures the selected action is visible by adjusting scroll offset.
    fn ensure_visible(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + MAX_VISIBLE_ITEMS {
            self.scroll_offset = self.selected.saturating_sub(MAX_VISIBLE_ITEMS - 1);
        }
    }

    /// Returns the currently selected action, if any.
    pub fn selected_action(&self) -> Option<&CodeActionOrCommand> {
        self.actions.get(self.selected)
    }

    /// Accepts the currently selected action, returning the result to apply.
    ///
    /// Returns `None` if no action is selected or the list is empty.
    pub fn accept_selected(&self) -> Option<CodeActionResult> {
        let action = self.selected_action()?;

        match action {
            CodeActionOrCommand::CodeAction(ca) => {
                let edit = ca.edit.clone();
                let command = ca.command.clone();

                match (edit, command) {
                    (Some(e), Some(c)) => Some(CodeActionResult::EditAndCommand(e, c)),
                    (Some(e), None) => Some(CodeActionResult::Edit(e)),
                    (None, Some(c)) => Some(CodeActionResult::Command(c)),
                    (None, None) => None, // Needs resolving, not supported yet
                }
            }
            CodeActionOrCommand::Command(cmd) => Some(CodeActionResult::Command(cmd.clone())),
        }
    }

    /// Returns the title/label for an action.
    fn action_title(action: &CodeActionOrCommand) -> &str {
        match action {
            CodeActionOrCommand::CodeAction(ca) => &ca.title,
            CodeActionOrCommand::Command(cmd) => &cmd.title,
        }
    }

    /// Returns the kind for an action (for display purposes).
    fn action_kind_label(action: &CodeActionOrCommand) -> Option<&str> {
        match action {
            CodeActionOrCommand::CodeAction(ca) => ca.kind.as_ref().map(|k| k.as_str()),
            CodeActionOrCommand::Command(_) => Some("command"),
        }
    }

    /// Calculates the preferred dimensions for the popup.
    fn content_size(&self) -> (u16, u16) {
        let visible_count = self.actions.len().min(MAX_VISIBLE_ITEMS);

        // Calculate width based on longest action title
        let max_title_width = self
            .actions
            .iter()
            .map(|a| Self::action_title(a).len() + 4) // +4 for icon and spacing
            .max()
            .unwrap_or(MIN_WIDTH as usize);

        let width = (max_title_width as u16 + 2).clamp(MIN_WIDTH, MAX_WIDTH);
        let height = (visible_count as u16 + 2).max(3); // +2 for border

        (width, height)
    }
}

impl Popup for CodeActionsPopup {
    fn id(&self) -> &str {
        "lsp-code-actions"
    }

    fn anchor(&self) -> PopupAnchor {
        self.anchor
    }

    fn size_hints(&self) -> SizeHints {
        let (width, height) = self.content_size();
        SizeHints {
            min_width: MIN_WIDTH,
            min_height: 3,
            max_width: MAX_WIDTH,
            max_height: (MAX_VISIBLE_ITEMS + 2) as u16,
            preferred_width: width,
            preferred_height: height,
        }
    }

    fn handle_event(&mut self, event: PopupEvent) -> PopupEventResult {
        match event {
            PopupEvent::Key(key) => self.handle_key(key),
            PopupEvent::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => {
                    self.select_prev();
                    PopupEventResult::consumed()
                }
                MouseEventKind::ScrollDown => {
                    self.select_next();
                    PopupEventResult::consumed()
                }
                _ => PopupEventResult::consumed(),
            },
            PopupEvent::CursorMoved => PopupEventResult::dismissed(),
            PopupEvent::Dismiss => PopupEventResult::dismissed(),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        if area.width < 3 || area.height < 3 {
            return;
        }

        // Clear the area with background color
        let mut buffer = Buffer::empty(area);
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buffer.cell_mut((x, y)) {
                    cell.set_symbol(" ")
                        .set_bg(theme.colors.popup.bg)
                        .set_fg(theme.colors.popup.fg);
                }
            }
        }

        // Draw border
        draw_border(&mut buffer, area, theme);

        // Draw title
        let title = " Code Actions ";
        let title_area = Rect::new(area.x + 2, area.y, area.width.saturating_sub(4), 1);
        if title_area.width > 0 {
            let title_line = Line::from(title).fg(theme.colors.popup.title);
            title_line.render(title_area, &mut buffer);
        }

        // Render content inside the border
        let content_area = Rect::new(
            area.x + 1,
            area.y + 1,
            area.width.saturating_sub(2),
            area.height.saturating_sub(2),
        );

        if content_area.width > 0 && content_area.height > 0 {
            self.render_actions(&mut buffer, content_area, theme);
        }

        // Render scroll indicators if needed
        self.render_scroll_indicators(&mut buffer, area, theme);

        // Merge buffer into frame
        frame.render_widget(BufferWidget(buffer), area);
    }

    fn is_modal(&self) -> bool {
        true
    }

    fn dismiss_on_cursor_move(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl CodeActionsPopup {
    /// Handles key events for the code actions popup.
    fn handle_key(&mut self, key: KeyEvent) -> PopupEventResult {
        match key.code {
            // Navigation
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                PopupEventResult::consumed()
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_prev();
                PopupEventResult::consumed()
            }

            // Accept selection
            KeyCode::Enter => PopupEventResult {
                consumed: true,
                dismiss: true,
            },

            // Dismiss without accepting
            KeyCode::Escape | KeyCode::Char('q') => PopupEventResult::dismissed(),

            // Let other keys dismiss
            _ => PopupEventResult::dismissed(),
        }
    }

    /// Renders the code action items.
    fn render_actions(&self, buffer: &mut Buffer, area: Rect, theme: &Theme) {
        for (i, action) in self
            .actions
            .iter()
            .skip(self.scroll_offset)
            .take(area.height as usize)
            .enumerate()
        {
            let is_selected = self.scroll_offset + i == self.selected;
            let line_area = Rect::new(area.x, area.y + i as u16, area.width, 1);
            self.render_action(buffer, line_area, action, is_selected, theme);
        }

        // Fill remaining lines with empty space if needed
        let rendered = self.actions.len().saturating_sub(self.scroll_offset).min(area.height as usize);
        for i in rendered..area.height as usize {
            let y = area.y + i as u16;
            for x in area.x..area.x + area.width {
                if let Some(cell) = buffer.cell_mut((x, y)) {
                    cell.set_symbol(" ").set_bg(theme.colors.popup.bg);
                }
            }
        }
    }

    /// Renders a single code action.
    fn render_action(
        &self,
        buffer: &mut Buffer,
        area: Rect,
        action: &CodeActionOrCommand,
        is_selected: bool,
        theme: &Theme,
    ) {
        let bg = if is_selected {
            theme.colors.popup.selection
        } else {
            theme.colors.popup.bg
        };

        let fg = theme.colors.popup.fg;

        // Get icon and color for the action kind
        let (icon, icon_color) = code_action_kind_icon(action);
        let title = Self::action_title(action);

        // Build the line: icon + title
        let icon_span = Span::styled(format!("{} ", icon), Style::default().fg(icon_color).bg(bg));
        let title_span = Span::styled(title, Style::default().fg(fg).bg(bg));

        let mut spans = vec![icon_span, title_span];

        // Add kind label if there's room
        if let Some(kind) = Self::action_kind_label(action) {
            let title_len = title.len() + 3; // icon + spaces
            let remaining = (area.width as usize).saturating_sub(title_len + 3);
            if remaining > 5 && kind.len() < remaining {
                let kind_text = format!(" [{}]", kind);
                spans.push(Span::styled(
                    kind_text,
                    Style::default()
                        .fg(theme.colors.popup.border)
                        .bg(bg)
                        .add_modifier(Modifier::DIM),
                ));
            }
        }

        let line = Line::from(spans);

        // Clear line first
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.cell_mut((x, area.y)) {
                cell.set_symbol(" ").set_bg(bg).set_fg(fg);
            }
        }

        // Render the line
        line.render(area, buffer);
    }

    /// Renders scroll indicators if content exceeds visible area.
    fn render_scroll_indicators(&self, buffer: &mut Buffer, area: Rect, theme: &Theme) {
        if self.actions.len() <= MAX_VISIBLE_ITEMS {
            return;
        }

        let style = Style::default()
            .fg(theme.colors.popup.border)
            .bg(theme.colors.popup.bg);

        // Up arrow if scrolled down
        if self.scroll_offset > 0 {
            if let Some(cell) = buffer.cell_mut((area.x + area.width - 2, area.y)) {
                cell.set_symbol("^").set_style(style);
            }
        }

        // Down arrow if more items below
        if self.scroll_offset + MAX_VISIBLE_ITEMS < self.actions.len() {
            if let Some(cell) = buffer.cell_mut((area.x + area.width - 2, area.y + area.height - 1))
            {
                cell.set_symbol("v").set_style(style);
            }
        }
    }
}

/// Returns the icon and color for a code action kind.
fn code_action_kind_icon(action: &CodeActionOrCommand) -> (&'static str, Color) {
    match action {
        CodeActionOrCommand::CodeAction(ca) => match &ca.kind {
            Some(kind) => {
                let kind_str = kind.as_str();
                if kind_str.starts_with("quickfix") {
                    ("", Color::Yellow) // Wrench/fix icon
                } else if kind_str.starts_with("refactor.extract") {
                    ("", Color::Blue) // Extract icon
                } else if kind_str.starts_with("refactor.inline") {
                    ("", Color::Blue) // Inline icon
                } else if kind_str.starts_with("refactor.rewrite") {
                    ("", Color::Blue) // Rewrite icon
                } else if kind_str.starts_with("refactor") {
                    ("", Color::Cyan) // Refactor icon
                } else if kind_str.starts_with("source.organizeImports") {
                    ("", Color::Green) // Import icon
                } else if kind_str.starts_with("source") {
                    ("", Color::Magenta) // Source icon
                } else {
                    ("", Color::White) // Generic action
                }
            }
            None => ("", Color::White),
        },
        CodeActionOrCommand::Command(_) => ("", Color::LightMagenta), // Command icon
    }
}

/// Helper to draw a border around an area.
fn draw_border(buffer: &mut Buffer, area: Rect, theme: &Theme) {
    let style = Style::default()
        .fg(theme.colors.popup.border)
        .bg(theme.colors.popup.bg);

    let x = area.x;
    let y = area.y;
    let width = area.width;
    let height = area.height;

    // Corners
    if let Some(cell) = buffer.cell_mut((x, y)) {
        cell.set_symbol(border::ROUNDED.top_left).set_style(style);
    }
    if let Some(cell) = buffer.cell_mut((x + width - 1, y)) {
        cell.set_symbol(border::ROUNDED.top_right).set_style(style);
    }
    if let Some(cell) = buffer.cell_mut((x, y + height - 1)) {
        cell.set_symbol(border::ROUNDED.bottom_left)
            .set_style(style);
    }
    if let Some(cell) = buffer.cell_mut((x + width - 1, y + height - 1)) {
        cell.set_symbol(border::ROUNDED.bottom_right)
            .set_style(style);
    }

    // Horizontal edges
    for xi in (x + 1)..(x + width - 1) {
        if let Some(cell) = buffer.cell_mut((xi, y)) {
            cell.set_symbol(border::ROUNDED.horizontal_top)
                .set_style(style);
        }
        if let Some(cell) = buffer.cell_mut((xi, y + height - 1)) {
            cell.set_symbol(border::ROUNDED.horizontal_bottom)
                .set_style(style);
        }
    }

    // Vertical edges
    for yi in (y + 1)..(y + height - 1) {
        if let Some(cell) = buffer.cell_mut((x, yi)) {
            cell.set_symbol(border::ROUNDED.vertical_left)
                .set_style(style);
        }
        if let Some(cell) = buffer.cell_mut((x + width - 1, yi)) {
            cell.set_symbol(border::ROUNDED.vertical_right)
                .set_style(style);
        }
    }
}

/// Widget wrapper to render a buffer directly.
struct BufferWidget(Buffer);

impl Widget for BufferWidget {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        // Merge source buffer into target
        for y in 0..self.0.area().height {
            for x in 0..self.0.area().width {
                let src_x = self.0.area().x + x;
                let src_y = self.0.area().y + y;
                if let Some(cell) = self.0.cell((src_x, src_y))
                    && let Some(target) = buf.cell_mut((src_x, src_y))
                {
                    *target = cell.clone();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_code_action(title: &str, kind: Option<&str>) -> CodeActionOrCommand {
        CodeActionOrCommand::CodeAction(CodeAction {
            title: title.to_string(),
            kind: kind.map(|k| CodeActionKind::new(k.to_string())),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: None,
            disabled: None,
            data: None,
        })
    }

    fn make_command(title: &str) -> CodeActionOrCommand {
        CodeActionOrCommand::Command(Command {
            title: title.to_string(),
            command: "test.command".to_string(),
            arguments: None,
        })
    }

    #[test]
    fn test_code_actions_popup_creation() {
        let actions = vec![
            make_code_action("Fix import", Some("quickfix")),
            make_code_action("Extract function", Some("refactor.extract")),
        ];

        let popup = CodeActionsPopup::new(actions).unwrap();
        assert_eq!(popup.id(), "lsp-code-actions");
        assert!(popup.has_actions());
        assert_eq!(popup.action_count(), 2);
    }

    #[test]
    fn test_code_actions_popup_empty() {
        let popup = CodeActionsPopup::new(vec![]);
        assert!(popup.is_none());
    }

    #[test]
    fn test_navigation() {
        let actions = vec![
            make_code_action("Action 1", None),
            make_code_action("Action 2", None),
            make_code_action("Action 3", None),
        ];

        let mut popup = CodeActionsPopup::new(actions).unwrap();
        assert_eq!(CodeActionsPopup::action_title(popup.selected_action().unwrap()), "Action 1");

        popup.select_next();
        assert_eq!(CodeActionsPopup::action_title(popup.selected_action().unwrap()), "Action 2");

        popup.select_next();
        assert_eq!(CodeActionsPopup::action_title(popup.selected_action().unwrap()), "Action 3");

        popup.select_next(); // Wraps around
        assert_eq!(CodeActionsPopup::action_title(popup.selected_action().unwrap()), "Action 1");

        popup.select_prev(); // Wraps around backward
        assert_eq!(CodeActionsPopup::action_title(popup.selected_action().unwrap()), "Action 3");
    }

    #[test]
    fn test_sorting_by_kind() {
        let actions = vec![
            make_code_action("Refactor", Some("refactor.extract")),
            make_code_action("Source action", Some("source.organizeImports")),
            make_code_action("Quickfix", Some("quickfix")),
            make_command("Command"),
        ];

        let popup = CodeActionsPopup::new(actions).unwrap();

        // Quickfix should be first
        assert_eq!(CodeActionsPopup::action_title(popup.selected_action().unwrap()), "Quickfix");
    }

    #[test]
    fn test_accept_with_edit() {
        let edit = WorkspaceEdit {
            changes: None,
            document_changes: None,
            change_annotations: None,
        };

        let action = CodeActionOrCommand::CodeAction(CodeAction {
            title: "Fix".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: None,
            edit: Some(edit),
            command: None,
            is_preferred: None,
            disabled: None,
            data: None,
        });

        let popup = CodeActionsPopup::new(vec![action]).unwrap();
        let result = popup.accept_selected();

        assert!(matches!(result, Some(CodeActionResult::Edit(_))));
    }

    #[test]
    fn test_accept_with_command() {
        let action = make_command("Run command");
        let popup = CodeActionsPopup::new(vec![action]).unwrap();
        let result = popup.accept_selected();

        assert!(matches!(result, Some(CodeActionResult::Command(_))));
    }
}
