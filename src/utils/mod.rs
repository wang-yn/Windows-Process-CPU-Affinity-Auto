pub mod error;
pub mod logger;
pub mod privilege;

pub use error::ServiceError;
pub use privilege::{is_elevated, require_administrator};
