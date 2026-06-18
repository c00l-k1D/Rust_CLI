pub mod processor;
pub mod ffi;

pub use processor::{process_data, ProcessError};
pub use ffi::*;
