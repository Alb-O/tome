//! Action result handler registry.
//!
//! Handlers are registered at compile-time and dispatched based on the
//! ActionResult variant they handle.

use linkme::distributed_slice;

use crate::ext::actions::ActionResult;
use super::EditorContext;

/// Registry of action result handlers.
#[distributed_slice]
pub static RESULT_HANDLERS: [ResultHandler];

/// Outcome of handling an action result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleOutcome {
    /// Result was handled, continue running.
    Handled,
    /// Result was handled, editor should quit.
    Quit,
    /// This handler doesn't handle this result type.
    NotHandled,
}

/// A handler for a specific ActionResult variant.
pub struct ResultHandler {
    /// Name for debugging/logging.
    pub name: &'static str,
    /// Which ActionResult variant(s) this handler processes.
    pub handles: fn(&ActionResult) -> bool,
    /// Handle the result, returning the outcome.
    pub handle: fn(&ActionResult, &mut EditorContext, bool) -> HandleOutcome,
}

impl std::fmt::Debug for ResultHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultHandler")
            .field("name", &self.name)
            .finish()
    }
}

/// Find and execute the handler for a result.
/// Returns true if the editor should quit.
pub fn dispatch_result(result: &ActionResult, ctx: &mut EditorContext, extend: bool) -> bool {
    for handler in RESULT_HANDLERS.iter() {
        if (handler.handles)(result) {
            match (handler.handle)(result, ctx, extend) {
                HandleOutcome::Handled => return false,
                HandleOutcome::Quit => return true,
                HandleOutcome::NotHandled => continue,
            }
        }
    }
    
    // Fallback: unhandled result
    ctx.message(&format!("Unhandled action result: {:?}", std::mem::discriminant(result)));
    false
}

/// Macro to simplify handler registration.
#[macro_export]
macro_rules! result_handler {
    ($static_name:ident, $name:literal, $variant:pat, $body:expr) => {
        #[::linkme::distributed_slice($crate::ext::editor_ctx::RESULT_HANDLERS)]
        static $static_name: $crate::ext::editor_ctx::ResultHandler = $crate::ext::editor_ctx::ResultHandler {
            name: $name,
            handles: |r| matches!(r, $variant),
            handle: $body,
        };
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handlers_registered() {
        // We should have handlers for all major result types
        let count = RESULT_HANDLERS.len();
        assert!(count >= 20, "expected at least 20 handlers, got {}", count);
    }

    #[test]
    fn test_handler_coverage() {
        // Verify we have handlers for common result types
        let has_ok = RESULT_HANDLERS.iter().any(|h| (h.handles)(&ActionResult::Ok));
        let has_quit = RESULT_HANDLERS.iter().any(|h| (h.handles)(&ActionResult::Quit));
        let has_error = RESULT_HANDLERS.iter().any(|h| (h.handles)(&ActionResult::Error("test".into())));
        
        assert!(has_ok, "missing handler for ActionResult::Ok");
        assert!(has_quit, "missing handler for ActionResult::Quit");
        assert!(has_error, "missing handler for ActionResult::Error");
    }
}
