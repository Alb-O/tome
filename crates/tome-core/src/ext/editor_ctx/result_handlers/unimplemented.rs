//! Handlers for not-yet-implemented features.
//!
//! These display a message to the user indicating the feature isn't available yet.

use crate::ext::actions::ActionResult;
use crate::ext::editor_ctx::HandleOutcome;
use crate::result_handler;

macro_rules! unimplemented_handler {
    ($slice:ident, $static_name:ident, $name:literal, $variant:pat, $msg:literal) => {
        result_handler!($slice, $static_name, $name, |r, ctx, _| {
            if matches!(r, $variant) {
                ctx.message($msg);
            }
            HandleOutcome::Handled
        });
    };
}

// SplitLines is handled via SelectionOpsAccess
result_handler!(RESULT_SPLIT_LINES_HANDLERS, HANDLE_SPLIT_LINES, "split_lines", |_, ctx, _| {
    if let Some(ops) = ctx.selection_ops() {
        ops.split_lines();
        HandleOutcome::Handled
    } else {
        ctx.message("Split lines not available");
        HandleOutcome::Handled
    }
});
unimplemented_handler!(RESULT_JUMP_FORWARD_HANDLERS, HANDLE_JUMP_FORWARD, "jump_forward", ActionResult::JumpForward, "Jump list not yet implemented");
unimplemented_handler!(RESULT_JUMP_BACKWARD_HANDLERS, HANDLE_JUMP_BACKWARD, "jump_backward", ActionResult::JumpBackward, "Jump list not yet implemented");
unimplemented_handler!(RESULT_SAVE_JUMP_HANDLERS, HANDLE_SAVE_JUMP, "save_jump", ActionResult::SaveJump, "Jump list not yet implemented");
unimplemented_handler!(RESULT_RECORD_MACRO_HANDLERS, HANDLE_RECORD_MACRO, "record_macro", ActionResult::RecordMacro, "Macros not yet implemented");
unimplemented_handler!(RESULT_PLAY_MACRO_HANDLERS, HANDLE_PLAY_MACRO, "play_macro", ActionResult::PlayMacro, "Macros not yet implemented");
unimplemented_handler!(RESULT_SAVE_SELECTIONS_HANDLERS, HANDLE_SAVE_SELECTIONS, "save_selections", ActionResult::SaveSelections, "Marks not yet implemented");
unimplemented_handler!(RESULT_RESTORE_SELECTIONS_HANDLERS, HANDLE_RESTORE_SELECTIONS, "restore_selections", ActionResult::RestoreSelections, "Marks not yet implemented");
unimplemented_handler!(RESULT_REPEAT_LAST_INSERT_HANDLERS, HANDLE_REPEAT_LAST_INSERT, "repeat_last_insert", ActionResult::RepeatLastInsert, "Repeat insert not yet implemented");
unimplemented_handler!(RESULT_REPEAT_LAST_OBJECT_HANDLERS, HANDLE_REPEAT_LAST_OBJECT, "repeat_last_object", ActionResult::RepeatLastObject, "Repeat object not yet implemented");
unimplemented_handler!(RESULT_DUPLICATE_SELECTIONS_DOWN_HANDLERS, HANDLE_DUPLICATE_DOWN, "duplicate_down", ActionResult::DuplicateSelectionsDown, "Duplicate down not yet implemented");
unimplemented_handler!(RESULT_DUPLICATE_SELECTIONS_UP_HANDLERS, HANDLE_DUPLICATE_UP, "duplicate_up", ActionResult::DuplicateSelectionsUp, "Duplicate up not yet implemented");
unimplemented_handler!(RESULT_MERGE_SELECTIONS_HANDLERS, HANDLE_MERGE_SELECTIONS, "merge_selections", ActionResult::MergeSelections, "Merge selections not yet implemented");
unimplemented_handler!(RESULT_ALIGN_HANDLERS, HANDLE_ALIGN, "align", ActionResult::Align, "Align not yet implemented");
unimplemented_handler!(RESULT_COPY_INDENT_HANDLERS, HANDLE_COPY_INDENT, "copy_indent", ActionResult::CopyIndent, "Copy indent not yet implemented");
unimplemented_handler!(RESULT_TABS_TO_SPACES_HANDLERS, HANDLE_TABS_TO_SPACES, "tabs_to_spaces", ActionResult::TabsToSpaces, "Tabs to spaces not yet implemented");
unimplemented_handler!(RESULT_SPACES_TO_TABS_HANDLERS, HANDLE_SPACES_TO_TABS, "spaces_to_tabs", ActionResult::SpacesToTabs, "Spaces to tabs not yet implemented");
unimplemented_handler!(RESULT_TRIM_SELECTIONS_HANDLERS, HANDLE_TRIM_SELECTIONS, "trim_selections", ActionResult::TrimSelections, "Trim selections not yet implemented");
