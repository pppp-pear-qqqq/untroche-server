pub trait Validation {
	fn validate(&self) -> Result<(), String>;
}

// Re-export the derive macro
#[doc(hidden)]
pub use derive::Validation;
