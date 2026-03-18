//! This library aids in command-line error reporting.
//!
//! # Example
//!
//! ```
//! use clerr::*;
//!
//! let code: Code = Code::warning("W012", "unused variable");
//! let info: TokenInfo = TokenInfo {
//!     severity: Severity::Warning,
//!     file_name: "src/main.rs",
//!     line: 8,
//!     position: 8,
//!     line_text: "    let x = 42;",
//!     token_len: 1,
//!     message: "consider prefixing with `_`",
//! };
//! let report: Report = Report::from(code).with_entry(info);
//! eprintln!("{}", report);
//! ```
//!
//! ```text
//! warning[W012]: unused variable
//!  --> src/main.rs [line=8, position=9]
//!   |
//! 8 |     let x = 42;
//!   |         ^ --- consider prefixing with `_`
//!   |
//! ```

#![allow(clippy::module_inception)]

pub use code::*;
pub use report::*;
pub use severity::*;

mod code;
mod report;
mod severity;
