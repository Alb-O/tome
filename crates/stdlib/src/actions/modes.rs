//! Mode-changing actions.

use evildoer_manifest::action;
use evildoer_manifest::actions::{ActionMode, ActionResult};

action!(window_mode, { description: "Enter window mode", bindings: r#"normal "ctrl-w""# },
	|_ctx| ActionResult::ModeChange(ActionMode::Window));
