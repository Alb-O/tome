use linkme::distributed_slice;
use ratatui::style::Color;

pub mod themes;

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct UiColors {
	pub bg: Color,
	pub fg: Color,
	pub gutter_fg: Color,
	pub cursor_bg: Color,
	pub cursor_fg: Color,
	pub selection_bg: Color,
	pub selection_fg: Color,
	pub message_fg: Color,
	pub command_input_fg: Color,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct StatusColors {
	pub normal_bg: Color,
	pub normal_fg: Color,
	pub insert_bg: Color,
	pub insert_fg: Color,
	pub goto_bg: Color,
	pub goto_fg: Color,
	pub view_bg: Color,
	pub view_fg: Color,
	pub command_bg: Color,
	pub command_fg: Color,

	pub dim_fg: Color,
	pub warning_fg: Color,
	pub error_fg: Color,
	pub success_fg: Color,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct PopupColors {
	pub bg: Color,
	pub fg: Color,
	pub border: Color,
	pub title: Color,
}

/// Per-semantic-style color pair for notifications.
/// If None, inherits from the base theme colors.
#[derive(Clone, Copy, Debug, Default)]
pub struct SemanticColorPair {
	pub bg: Option<Color>,
	pub fg: Option<Color>,
}

impl SemanticColorPair {
	/// Const default with no overrides (inherit all).
	pub const NONE: Self = Self { bg: None, fg: None };
}

/// Notification-specific color overrides.
/// All fields are optional; None means inherit from popup/status colors.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationColors {
	/// Default/normal notification style
	pub normal: SemanticColorPair,
	/// Info notifications (inherits from popup.fg if None)
	pub info: SemanticColorPair,
	/// Warning notifications (inherits from status.warning_fg if None)
	pub warning: SemanticColorPair,
	/// Error notifications (inherits from status.error_fg if None)
	pub error: SemanticColorPair,
	/// Success notifications (inherits from status.success_fg if None)
	pub success: SemanticColorPair,
	/// Dim/debug notifications (inherits from status.dim_fg if None)
	pub dim: SemanticColorPair,
	/// Border color override (inherits from popup.border if None)
	pub border: Option<Color>,
}

impl NotificationColors {
	/// Const default with no overrides (inherit all colors from popup/status).
	pub const INHERITED: Self = Self {
		normal: SemanticColorPair::NONE,
		info: SemanticColorPair::NONE,
		warning: SemanticColorPair::NONE,
		error: SemanticColorPair::NONE,
		success: SemanticColorPair::NONE,
		dim: SemanticColorPair::NONE,
		border: None,
	};
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct ThemeColors {
	pub ui: UiColors,
	pub status: StatusColors,
	pub popup: PopupColors,
	/// Notification-specific color overrides (optional, inherits from popup/status)
	pub notification: NotificationColors,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Theme {
	pub id: &'static str,
	pub name: &'static str,
	pub aliases: &'static [&'static str],
	pub colors: ThemeColors,
	pub priority: i16,
	pub source: tome_manifest::RegistrySource,
}

#[distributed_slice]
pub static THEMES: [Theme] = [..];

#[distributed_slice(THEMES)]
pub static DEFAULT_THEME: Theme = Theme {
	id: "default",
	name: "default",
	aliases: &[],
	colors: ThemeColors {
		ui: UiColors {
			bg: Color::Reset,
			fg: Color::Reset,
			gutter_fg: Color::DarkGray,
			cursor_bg: Color::White,
			cursor_fg: Color::Black,
			selection_bg: Color::Blue,
			selection_fg: Color::White,
			message_fg: Color::Yellow,
			command_input_fg: Color::White,
		},
		status: StatusColors {
			normal_bg: Color::Blue,
			normal_fg: Color::White,
			insert_bg: Color::Green,
			insert_fg: Color::Black,
			goto_bg: Color::Magenta,
			goto_fg: Color::White,
			view_bg: Color::Cyan,
			view_fg: Color::Black,
			command_bg: Color::Yellow,
			command_fg: Color::Black,

			dim_fg: Color::DarkGray,
			warning_fg: Color::Yellow,
			error_fg: Color::Red,
			success_fg: Color::Green,
		},
		popup: PopupColors {
			bg: Color::Rgb(10, 10, 10),
			fg: Color::White,
			border: Color::White,
			title: Color::Yellow,
		},
		notification: NotificationColors::INHERITED,
	},
	priority: 0,
	source: tome_manifest::RegistrySource::Builtin,
};

use ratatui::style::Style;
use tome_manifest::SemanticStyle;

impl ThemeColors {
	/// Resolve notification style for a given semantic style.
	/// Uses notification-specific overrides if set, otherwise inherits from popup/status colors.
	pub fn notification_style(&self, semantic: SemanticStyle) -> Style {
		let pair = match semantic {
			SemanticStyle::Normal => &self.notification.normal,
			SemanticStyle::Info => &self.notification.info,
			SemanticStyle::Warning => &self.notification.warning,
			SemanticStyle::Error => &self.notification.error,
			SemanticStyle::Success => &self.notification.success,
			SemanticStyle::Dim => &self.notification.dim,
		};

		// Resolve background: notification override -> popup.bg
		let bg = pair.bg.unwrap_or(self.popup.bg);

		// Resolve foreground: notification override -> semantic fallback from status/popup
		let fg = pair.fg.unwrap_or_else(|| match semantic {
			SemanticStyle::Normal | SemanticStyle::Info => self.popup.fg,
			SemanticStyle::Warning => self.status.warning_fg,
			SemanticStyle::Error => self.status.error_fg,
			SemanticStyle::Success => self.status.success_fg,
			SemanticStyle::Dim => self.status.dim_fg,
		});

		Style::default().bg(bg).fg(fg)
	}

	/// Resolve notification border color.
	pub fn notification_border(&self) -> Color {
		self.notification.border.unwrap_or(self.popup.border)
	}
}

pub fn get_theme(name: &str) -> Option<&'static Theme> {
	let normalize = |s: &str| -> String {
		s.chars()
			.filter(|c| *c != '-' && *c != '_')
			.collect::<String>()
			.to_lowercase()
	};

	let search = normalize(name);

	THEMES
		.iter()
		.find(|t| normalize(t.name) == search || t.aliases.iter().any(|a| normalize(a) == search))
}

pub fn blend_colors(fg: Color, bg: Color, alpha: f32) -> Color {
	let fg_rgb = match fg {
		Color::Rgb(r, g, b) => (r, g, b),
		_ => return fg, // Fallback for non-RGB colors
	};

	let bg_rgb = match bg {
		Color::Rgb(r, g, b) => (r, g, b),
		_ => return fg, // Fallback if background is unknown/non-RGB
	};

	let r = (fg_rgb.0 as f32 * alpha + bg_rgb.0 as f32 * (1.0 - alpha)) as u8;
	let g = (fg_rgb.1 as f32 * alpha + bg_rgb.1 as f32 * (1.0 - alpha)) as u8;
	let b = (fg_rgb.2 as f32 * alpha + bg_rgb.2 as f32 * (1.0 - alpha)) as u8;

	Color::Rgb(r, g, b)
}

pub fn suggest_theme(name: &str) -> Option<&'static str> {
	let name = name.to_lowercase();
	let mut best_match = None;
	let mut best_score = 0.0;

	for theme in THEMES {
		let score = strsim::jaro_winkler(&name, theme.name);
		if score > best_score {
			best_score = score;
			best_match = Some(theme.name);
		}

		for alias in theme.aliases {
			let score = strsim::jaro_winkler(&name, alias);
			if score > best_score {
				best_score = score;
				best_match = Some(theme.name);
			}
		}
	}

	if best_score > 0.8 { best_match } else { None }
}

use futures::future::LocalBoxFuture;
use tome_manifest::editor_ctx::MessageAccess;
use tome_manifest::{
	COMMANDS, CommandContext, CommandDef, CommandError, CommandOutcome, OPTIONS, OptionDef,
	OptionScope, OptionType, OptionValue,
};

pub const DEFAULT_THEME_ID: &str = "gruvbox";

#[distributed_slice(OPTIONS)]
pub static OPT_THEME: OptionDef = OptionDef {
	id: "theme",
	name: "theme",
	description: "Editor color theme",
	value_type: OptionType::String,
	default: || OptionValue::String(DEFAULT_THEME_ID.to_string()),
	scope: OptionScope::Global,
	source: tome_manifest::RegistrySource::Builtin,
};

fn cmd_theme<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let theme_name = ctx
			.args
			.first()
			.ok_or(CommandError::MissingArgument("theme name"))?;
		// TODO: Implement theme access trait in EditorOps
		ctx.notify(
			"info",
			&format!("Theme command not yet implemented: {}", theme_name),
		);
		Ok(CommandOutcome::Ok)
	})
}

#[distributed_slice(COMMANDS)]
pub static CMD_THEME: CommandDef = CommandDef {
	id: "theme",
	name: "theme",
	aliases: &["colorscheme"],
	description: "Set the editor theme",
	handler: cmd_theme,
	user_data: None,
	priority: 0,
	source: tome_manifest::RegistrySource::Builtin,
	required_caps: &[],
	flags: tome_manifest::flags::NONE,
};
