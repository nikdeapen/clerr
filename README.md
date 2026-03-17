# clerr

[![Crates.io](https://img.shields.io/crates/v/clerr.svg)](https://crates.io/crates/clerr)
[![Docs.rs](https://docs.rs/clerr/badge.svg)](https://docs.rs/clerr)
[![License: MIT](https://img.shields.io/crates/l/clerr.svg)](https://opensource.org/licenses/MIT)

This library aids in command-line error reporting.

    clerr = "0.10.0-rc.2"

### Dependencies

- [colored](https://crates.io/crates/colored)

## Examples

### Simple Report

```rust,ignore
use clerr::*;

let code: Code = Code::error("E001", "file not found");
let report: Report = Report::from(code);
eprintln!("{}", report);
```

```text
error[E001]: file not found
```

### Token Info

Highlight a specific token in a source line with a message, similar to `rustc` output.

```rust,ignore
use clerr::*;

let code: Code = Code::warning("W012", "unused variable");
let info: TokenInfo = TokenInfo {
    severity: Severity::Warning,
    file_name: "src/main.rs",
    line: 8,
    position: 4,
    line_text: "    let x = 42;",
    token_len: 1,
    message: "consider prefixing with `_`",
};
let report: Report = Report::from(code).with_entry(info);
eprintln!("{}", report);
```

```text
warning[W012]: unused variable
 --> src/main.rs [line=8, position=5]
  |
8 |     let x = 42;
  |         ^ --- consider prefixing with `_`
  |
```

### Properties

Display aligned key-value pairs under a report.

```rust,ignore
use clerr::*;

let props: Properties = Properties::default()
    .with_property("file", "/etc/config.yml")
    .with_property("expected", "utf-8")
    .with_property("found", "binary");
let code: Code = Code::error("E042", "invalid encoding");
let report: Report = Report::from(code).with_entry(props);
eprintln!("{}", report);
```

```text
error[E042]: invalid encoding
    file:      /etc/config.yml
    expected:  utf-8
    found:     binary
```

### Severity Levels

Three severity levels are available, each with a distinct color:

| Level     | Color  |
|-----------|--------|
| `Error`   | Red    |
| `Warning` | Yellow |
| `Info`    | Blue   |

These colors are hardcoded for the colorblind.

## Issues & Contributing

See [ISSUES.md](ISSUES.md) for future work and [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
