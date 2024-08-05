#![doc = include_str ! ("../README.md")]

#[cfg(feature = "windows")]
pub mod devices;
#[cfg(feature = "windows")]
pub mod duplication;
#[cfg(feature = "windows")]
pub mod error;
#[cfg(feature = "windows")]
pub mod outputs;
#[cfg(feature = "windows")]
pub mod tex_reader;
#[cfg(feature = "windows")]
pub mod texture;
pub mod types;
#[cfg(feature = "windows")]
mod utils;

#[cfg(feature = "windows")]
pub use duplication::*;
#[cfg(feature = "windows")]
pub use utils::{co_init, set_process_dpi_awareness};

#[cfg(feature = "windows")]
pub type Result<T> = core::result::Result<T, crate::error::DDApiError>;
