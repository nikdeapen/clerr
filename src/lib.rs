//! A library for colored command-line error, warning, and info reporting.
//!
//! # Example
//!
//! ```
//! use clerr::*;
//!
//! let code: Code = Code::error("E001", "unexpected token");
//! let token_info: TokenInfo = TokenInfo {
//!     file_name: "src/main.rs",
//!     line: 12,
//!     position: 4,
//!     line_text: "let x = @;",
//!     token_len: 1,
//!     severity: Severity::Error,
//!     message: "expected expression",
//! };
//! let properties: Properties = Properties::default()
//!     .with_property("expected", "expression")
//!     .with_property("found", "@");
//! let report: Report = Report::from(code)
//!     .with_entry(token_info)
//!     .with_entry(properties);
//! eprintln!("{}", report);
//! ```

#![allow(clippy::module_inception)]

pub use code::*;
pub use report::*;
pub use severity::*;

mod code;
mod report;
mod severity;
