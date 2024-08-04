#![doc = include_str ! ("../README.md")]

#[cfg(not(feature = "cross_platform_types"))]
pub mod devices;
#[cfg(not(feature = "cross_platform_types"))]
pub mod duplication;
#[cfg(not(feature = "cross_platform_types"))]
pub mod error;
#[cfg(not(feature = "cross_platform_types"))]
pub mod outputs;
#[cfg(not(feature = "cross_platform_types"))]
pub mod tex_reader;
#[cfg(not(feature = "cross_platform_types"))]
pub mod texture;
pub mod types;
#[cfg(not(feature = "cross_platform_types"))]
mod utils;

#[cfg(not(feature = "cross_platform_types"))]
pub use duplication::*;
#[cfg(not(feature = "cross_platform_types"))]
pub use utils::{co_init, set_process_dpi_awareness};

#[cfg(not(feature = "cross_platform_types"))]
pub type Result<T> = core::result::Result<T, crate::error::DDApiError>;
