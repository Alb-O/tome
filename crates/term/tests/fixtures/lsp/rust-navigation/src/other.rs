use crate::shared_function;
use crate::helper_function;

/// First function that calls shared_function.
pub fn caller_one() -> i32 {
    shared_function() // Reference 1
}

/// Second function that calls shared_function.
pub fn caller_two() -> i32 {
    shared_function() // Reference 2
}

/// Function that uses helper_function.
pub fn caller_three() -> i32 {
    helper_function(10)
}
