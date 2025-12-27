//! Toast notification widget.

use alloc::string::String;

use crate::layout::Alignment;
use crate::style::Style;
use crate::widgets::block::Padding;
use crate::widgets::{Block, BorderType, Borders};

use super::types::{Anchor, Animation, AutoDismiss, Level, SizeConstraint, SlideDirection, Timing};

/// A toast notification to display to the user.
///
/// Toasts are transient messages that appear briefly and can auto-dismiss.
/// They support different severity levels, animations, and positioning.
#[derive(Debug, Clone)]
pub struct Toast {
    /// Main message content.
    pub content: String,
    /// Optional title.
    pub title: Option<String>,
    /// Severity level.
    pub level: Level,
    /// Screen anchor position.
    pub anchor: Anchor,
    /// Animation style.
    pub animation: Animation,
    /// Slide direction (for slide animations).
    pub slide_direction: SlideDirection,
    /// Entry animation duration.
    pub entry_timing: Timing,
    /// Dwell duration.
    pub dwell_timing: Timing,
    /// Exit animation duration.
    pub exit_timing: Timing,
    /// Auto-dismiss behavior.
    pub auto_dismiss: AutoDismiss,
    /// Maximum width constraint.
    pub max_width: Option<SizeConstraint>,
    /// Maximum height constraint.
    pub max_height: Option<SizeConstraint>,
    /// Internal padding.
    pub padding: Padding,
    /// External margin from anchor.
    pub margin: u16,
    /// Background/block style.
    pub style: Style,
    /// Border style.
    pub border_style: Style,
    /// Title style.
    pub title_style: Style,
    /// Border type.
    pub border_type: BorderType,
    /// Whether to apply fade effect during slide/expand animations.
    pub fade_effect: bool,
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            content: String::new(),
            title: None,
            level: Level::Info,
            anchor: Anchor::BottomRight,
            animation: Animation::Fade,
            slide_direction: SlideDirection::Auto,
            entry_timing: Timing::Auto,
            dwell_timing: Timing::Auto,
            exit_timing: Timing::Auto,
            auto_dismiss: AutoDismiss::default(),
            max_width: Some(SizeConstraint::Percent(0.4)),
            max_height: Some(SizeConstraint::Percent(0.4)),
            padding: Padding::horizontal(1),
            margin: 1,
            style: Style::default(),
            border_style: Style::default(),
            title_style: Style::default(),
            border_type: BorderType::Rounded,
            fade_effect: false,
        }
    }
}

impl Toast {
    /// Creates a new toast with the given content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Sets the title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the severity level.
    #[must_use]
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Sets the anchor position.
    #[must_use]
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Sets the animation style.
    #[must_use]
    pub fn animation(mut self, animation: Animation) -> Self {
        self.animation = animation;
        self
    }

    /// Sets the slide direction.
    #[must_use]
    pub fn slide_direction(mut self, direction: SlideDirection) -> Self {
        self.slide_direction = direction;
        self
    }

    /// Sets the auto-dismiss behavior.
    #[must_use]
    pub fn auto_dismiss(mut self, auto_dismiss: AutoDismiss) -> Self {
        self.auto_dismiss = auto_dismiss;
        self
    }

    /// Sets the background style.
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the border style.
    #[must_use]
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Sets the border type.
    #[must_use]
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Sets the padding.
    #[must_use]
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the margin from anchor.
    #[must_use]
    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    /// Enables/disables fade effect during animations.
    #[must_use]
    pub fn fade_effect(mut self, enabled: bool) -> Self {
        self.fade_effect = enabled;
        self
    }

    /// Creates a block for rendering this toast.
    pub(crate) fn to_block(&self) -> Block<'_> {
        let mut block = Block::default()
            .style(self.style)
            .borders(Borders::ALL)
            .border_type(self.border_type)
            .border_style(self.border_style)
            .padding(self.padding);

        if let Some(ref title) = self.title {
            block = block.title(
                crate::text::Line::raw(title.as_str())
                    .alignment(Alignment::Center)
                    .style(self.title_style),
            );
        }

        block
    }
}
