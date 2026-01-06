/// A shared function that is called from multiple places.
///
/// This function is used to test go-to-definition and find-references.
pub fn shared_function() -> i32 {
    42
}

/// Another function for navigation testing.
pub fn helper_function(x: i32) -> i32 {
    x * 2
}

/// Test function that calls helper_function.
/// Used for testing go-to-definition within the same file.
pub fn test_caller() -> i32 {
    // Call helper_function here - gd on this should jump to line 9
    helper_function(5)
}

mod other;
