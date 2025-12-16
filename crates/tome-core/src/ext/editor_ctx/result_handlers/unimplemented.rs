//! Handlers for not-yet-implemented features.
//!
//! These display a message to the user indicating the feature isn't available yet.

use linkme::distributed_slice;

use crate::ext::actions::ActionResult;
use crate::ext::editor_ctx::{HandleOutcome, ResultHandler, RESULT_HANDLERS};

macro_rules! unimplemented_handler {
    ($static_name:ident, $name:literal, $variant:pat, $msg:literal) => {
        #[distributed_slice(RESULT_HANDLERS)]
        static $static_name: ResultHandler = ResultHandler {
            name: $name,
            handles: |r| matches!(r, $variant),
            handle: |_, ctx, _| {
                ctx.message($msg);
                HandleOutcome::Handled
            },
        };
    };
}

unimplemented_handler!(HANDLE_SPLIT_LINES, "split_lines", ActionResult::SplitLines, "Split lines not yet implemented");
unimplemented_handler!(HANDLE_JUMP_FORWARD, "jump_forward", ActionResult::JumpForward, "Jump list not yet implemented");
unimplemented_handler!(HANDLE_JUMP_BACKWARD, "jump_backward", ActionResult::JumpBackward, "Jump list not yet implemented");
unimplemented_handler!(HANDLE_SAVE_JUMP, "save_jump", ActionResult::SaveJump, "Jump list not yet implemented");
unimplemented_handler!(HANDLE_RECORD_MACRO, "record_macro", ActionResult::RecordMacro, "Macros not yet implemented");
unimplemented_handler!(HANDLE_PLAY_MACRO, "play_macro", ActionResult::PlayMacro, "Macros not yet implemented");
unimplemented_handler!(HANDLE_SAVE_SELECTIONS, "save_selections", ActionResult::SaveSelections, "Marks not yet implemented");
unimplemented_handler!(HANDLE_RESTORE_SELECTIONS, "restore_selections", ActionResult::RestoreSelections, "Marks not yet implemented");
unimplemented_handler!(HANDLE_REPEAT_LAST_INSERT, "repeat_last_insert", ActionResult::RepeatLastInsert, "Repeat insert not yet implemented");
unimplemented_handler!(HANDLE_REPEAT_LAST_OBJECT, "repeat_last_object", ActionResult::RepeatLastObject, "Repeat object not yet implemented");
unimplemented_handler!(HANDLE_DUPLICATE_DOWN, "duplicate_down", ActionResult::DuplicateSelectionsDown, "Duplicate down not yet implemented");
unimplemented_handler!(HANDLE_DUPLICATE_UP, "duplicate_up", ActionResult::DuplicateSelectionsUp, "Duplicate up not yet implemented");
unimplemented_handler!(HANDLE_MERGE_SELECTIONS, "merge_selections", ActionResult::MergeSelections, "Merge selections not yet implemented");
unimplemented_handler!(HANDLE_ALIGN, "align", ActionResult::Align, "Align not yet implemented");
unimplemented_handler!(HANDLE_COPY_INDENT, "copy_indent", ActionResult::CopyIndent, "Copy indent not yet implemented");
unimplemented_handler!(HANDLE_TABS_TO_SPACES, "tabs_to_spaces", ActionResult::TabsToSpaces, "Tabs to spaces not yet implemented");
unimplemented_handler!(HANDLE_SPACES_TO_TABS, "spaces_to_tabs", ActionResult::SpacesToTabs, "Spaces to tabs not yet implemented");
unimplemented_handler!(HANDLE_TRIM_SELECTIONS, "trim_selections", ActionResult::TrimSelections, "Trim selections not yet implemented");
