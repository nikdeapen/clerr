#![doc = include_str!("../README.md")]

pub use colored;

pub use code::*;
pub use report::*;
pub use severity::*;

mod code;
mod report;
mod severity;
