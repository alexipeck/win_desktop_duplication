#![doc = include_str ! ("../README.md")]

use crate::errors::DDApiError;

pub mod devices;
pub mod duplication;
pub mod errors;
pub mod outputs;
pub mod tex_reader;
pub mod texture;
mod utils;

pub use duplication::*;
pub use utils::{co_init, set_process_dpi_awareness};

pub type Result<T> = core::result::Result<T, DDApiError>;
