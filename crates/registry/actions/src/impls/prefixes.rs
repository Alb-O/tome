//! Key sequence prefix definitions for the which-key HUD.
//!
//! Prefixes can be nested to create hierarchical key sequences. Multi-key
//! prefixes use space-separated format and require an explicit identifier.

use crate::key_prefix;

key_prefix!(normal "g" => "Goto");
key_prefix!(normal "z" => "View");
key_prefix!(normal "ctrl-w" => "Window");
key_prefix!(normal "ctrl-w s" as ctrl_w_s => "Split");
key_prefix!(normal "ctrl-w f" as ctrl_w_f => "Focus");
key_prefix!(normal "ctrl-w b" as ctrl_w_b => "Buffer");
key_prefix!(normal "ctrl-w c" as ctrl_w_c => "Close");
