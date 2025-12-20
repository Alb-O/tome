use crate::notifications::types::Level;

const ICON_INFO: &str = "󰋼";
const ICON_WARN: &str = "󰀪";
const ICON_ERROR: &str = "󰅚";
const ICON_DEBUG: &str = "󰃭";
const ICON_TRACE: &str = "󰗋";

/// Returns the icon string for a given notification level.
///
/// # Arguments
///
/// * `level` - Optional notification level
///
/// # Returns
///
/// * `Some(&'static str)` - The icon string for the given level
/// * `None` - If no level is provided
///
/// # Examples
///
/// ```
/// use ratatui_notifications::notifications::functions::fnc_get_level_icon::get_level_icon;
/// use ratatui_notifications::notifications::types::Level;
///
/// assert_eq!(get_level_icon(Some(Level::Info)), Some("󰋼"));
/// assert_eq!(get_level_icon(Some(Level::Error)), Some("󰅚"));
/// assert_eq!(get_level_icon(None), None);
/// ```
pub fn get_level_icon(level: Option<Level>) -> Option<&'static str> {
	match level {
		Some(Level::Info) => Some(ICON_INFO),
		Some(Level::Warn) => Some(ICON_WARN),
		Some(Level::Error) => Some(ICON_ERROR),
		Some(Level::Debug) => Some(ICON_DEBUG),
		Some(Level::Trace) => Some(ICON_TRACE),
		None => None,
	}
}

