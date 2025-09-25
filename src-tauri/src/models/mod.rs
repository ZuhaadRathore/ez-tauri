//! Data models for the application.
//!
//! Contains all the data structures used throughout the application
//! including user models, logging structures, and configuration types.

pub mod logs;
pub mod settings;
pub mod user;

pub use logs::*;
#[allow(unused_imports)]
pub use settings::*;
pub use user::*;
