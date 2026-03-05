# clerr

A Rust library for colored command-line error, warning, and info reporting.

## Installation

```toml
[dependencies]
clerr = "0.10.0-rc.1"
```

### Dependencies

- [colored](https://crates.io/crates/colored)

## Usage

### Simple Report

```rust
use clerr::{Code, Report};

let code = Code::error("E001", "file not found");
let report = Report::new(code);
report.eprint();
```

```text
error[E001]: file not found
```

### Token Info

Highlight a specific token in a source line with a message, similar to `rustc` output.

```rust
use clerr::{Code, Report, Severity, TokenInfo};

let code = Code::warning("W012", "unused variable");
let info = TokenInfo {
    file_name: "src/main.rs",
    line: 8,           // 1-based line number
    position: 4,       // 0-based column position
    line_text: "    let x = 42;",
    token_len: 1,
    severity: Severity::Warning,
    message: "consider prefixing with `_`",
};
let report = Report::new(code).with_entry(info);
report.eprint();
```

```text
warning[W012]: unused variable
 --> src/main.rs [line=8, position=5]
 |
8 |     let x = 42;
 |     ^ --- consider prefixing with `_`
 |
```

### Properties

Display aligned key-value pairs under a report.

```rust
use clerr::{Code, Properties, Report};

let props = Properties::default()
    .with_property("file", "/etc/config.yml")
    .with_property("expected", "utf-8")
    .with_property("found", "binary");
let code = Code::error("E042", "invalid encoding");
let report = Report::new(code).with_entry(props);
report.eprint();
```

```text
error[E042]: invalid encoding
    file:      /etc/config.yml
    expected:  utf-8
    found:     binary
```

### Custom Entries

Implement the `ReportEntry` trait to create custom entry types.

```rust
use clerr::ReportEntry;
use colored::{ColoredString, Colorize};

struct Suggestion(String);

impl ReportEntry for Suggestion {
    fn entry(self) -> Vec<ColoredString> {
        vec!["    suggestion: ".blue(), self.0.normal()]
    }
}
```

### Severity Levels

Three severity levels are available, each with a distinct color:

| Level     | Color  |
|-----------|--------|
| `Error`   | Red    |
| `Warning` | Yellow |
| `Info`    | Blue   |

`Severity` implements `Ord` with `Info < Warning < Error`.

## Output

Reports are printed to stderr via `report.eprint()`, or formatted to any
`std::fmt::Formatter` via the `Display` trait. Both `Code` and `Report`
implement `std::error::Error`.

## License

MIT
