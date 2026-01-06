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

mod other;
