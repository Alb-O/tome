//! Command palette result handlers.

use xeno_registry::{HandleOutcome, result_handler};

result_handler!(
	RESULT_OPEN_PALETTE_HANDLERS,
	HANDLE_OPEN_PALETTE,
	"open_palette",
	|_, ctx, _| {
		ctx.open_palette();
		HandleOutcome::Handled
	}
);

result_handler!(
	RESULT_CLOSE_PALETTE_HANDLERS,
	HANDLE_CLOSE_PALETTE,
	"close_palette",
	|_, ctx, _| {
		ctx.close_palette();
		HandleOutcome::Handled
	}
);

result_handler!(
	RESULT_EXECUTE_PALETTE_HANDLERS,
	HANDLE_EXECUTE_PALETTE,
	"execute_palette",
	|_, ctx, _| {
		ctx.execute_palette();
		HandleOutcome::Handled
	}
);
