//! Notification-specific rendering helpers.

mod codegen;
mod layout;

#[cfg(test)]
mod tests;

pub use codegen::generate_code;
pub use layout::calculate_size;
