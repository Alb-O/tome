/// Terminal escape sequence identifiers for configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalSequence {
	EnableAlternateScreen,
	DisableAlternateScreen,
	EnableMouseTracking,
	DisableMouseTracking,
	EnableSgrMouse,
	DisableSgrMouse,
	EnableAnyEventMouse,
	DisableAnyEventMouse,
	PushKittyKeyboardDisambiguate,
	PopKittyKeyboardFlags,
	ResetCursorStyle,
}

/// Configures terminal feature sequences used by the UI.
#[derive(Debug, Clone, Copy)]
pub struct TerminalConfig {
	/// Sequences emitted when entering the editor UI.
	pub enter_sequences: &'static [TerminalSequence],
	/// Sequences emitted when exiting the editor UI.
	pub exit_sequences: &'static [TerminalSequence],
	/// Sequences emitted on panic cleanup.
	pub panic_sequences: &'static [TerminalSequence],
}

const TERMINAL_CONFIG_ENV: &str = "EVILDOER_TERMINAL_CONFIG";

const DEFAULT_ENTER: &[TerminalSequence] = &[
	TerminalSequence::EnableAlternateScreen,
	TerminalSequence::PushKittyKeyboardDisambiguate,
	TerminalSequence::EnableMouseTracking,
	TerminalSequence::EnableSgrMouse,
	TerminalSequence::EnableAnyEventMouse,
];

const DEFAULT_EXIT: &[TerminalSequence] = &[
	TerminalSequence::ResetCursorStyle,
	TerminalSequence::PopKittyKeyboardFlags,
	TerminalSequence::DisableMouseTracking,
	TerminalSequence::DisableSgrMouse,
	TerminalSequence::DisableAnyEventMouse,
	TerminalSequence::DisableAlternateScreen,
];

const NO_KITTY_ENTER: &[TerminalSequence] = &[
	TerminalSequence::EnableAlternateScreen,
	TerminalSequence::EnableMouseTracking,
	TerminalSequence::EnableSgrMouse,
	TerminalSequence::EnableAnyEventMouse,
];

const NO_KITTY_EXIT: &[TerminalSequence] = &[
	TerminalSequence::ResetCursorStyle,
	TerminalSequence::DisableMouseTracking,
	TerminalSequence::DisableSgrMouse,
	TerminalSequence::DisableAnyEventMouse,
	TerminalSequence::DisableAlternateScreen,
];

fn supports_kitty_keyboard() -> bool {
	if std::env::var_os("KITTY_WINDOW_ID").is_some()
		|| std::env::var_os("KITTY_LISTEN_ON").is_some()
	{
		return true;
	}

	std::env::var("TERM")
		.map(|term| term.contains("kitty"))
		.unwrap_or(false)
}

impl TerminalConfig {
	/// Creates a configuration with explicit sequences.
	pub const fn new(
		enter_sequences: &'static [TerminalSequence],
		exit_sequences: &'static [TerminalSequence],
		panic_sequences: &'static [TerminalSequence],
	) -> Self {
		Self {
			enter_sequences,
			exit_sequences,
			panic_sequences,
		}
	}

	/// Detects the best terminal configuration for the current environment.
	///
	/// Respects `EVILDOER_TERMINAL_CONFIG` overrides ("kitty" or "no-kitty").
	pub fn detect() -> Self {
		if let Some(config) = Self::from_env() {
			return config;
		}

		if supports_kitty_keyboard() {
			return Self::default();
		}

		Self::new(NO_KITTY_ENTER, NO_KITTY_EXIT, NO_KITTY_EXIT)
	}

	fn from_env() -> Option<Self> {
		let value = std::env::var(TERMINAL_CONFIG_ENV).ok()?;
		let value = value.trim().to_ascii_lowercase();
		match value.as_str() {
			"kitty" | "default" => Some(Self::default()),
			"no-kitty" | "basic" => Some(Self::new(NO_KITTY_ENTER, NO_KITTY_EXIT, NO_KITTY_EXIT)),
			_ => None,
		}
	}
}

impl Default for TerminalConfig {
	fn default() -> Self {
		Self::new(DEFAULT_ENTER, DEFAULT_EXIT, DEFAULT_EXIT)
	}
}
