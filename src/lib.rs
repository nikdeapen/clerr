//! A library for colored command-line error, warning, and info reporting.
//!
//! Build a [`Report`] from a [`Code`] and optional entries like [`TokenInfo`] and [`Properties`].
//! Use [`Report::eprint`] to print to stderr, or format with [`Display`](std::fmt::Display).

#![allow(clippy::module_inception)]

pub use code::*;
pub use report::*;
pub use severity::*;

mod code;
mod report;
mod severity;
