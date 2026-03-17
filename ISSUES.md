# Issues

## 1. Allocation Performance

The `colored` crate's `ColoredString` type owns its string data. When building reports, this can lead to many small
allocations. This is unlikely to be a real performance issue since info & error reports should not be super common.
There seem to be optimizations that can be made within the crate to reduce allocations as well.

## 2. Testing

The crate lacks assertion-based tests. The existing tests only use `println!` for manual inspection and cannot catch
regressions in output formatting. Tests should strip ANSI escape codes and assert on the plain-text output.

## 3. Report Equality Includes ANSI Escape Codes

`Report` derives `Eq`, `Ord`, and `Hash` but its `entries` field contains pre-rendered ANSI escape codes. This means
two semantically identical reports could compare as unequal if produced under different color settings. In practice this
is unlikely since color settings don't change within a single process run.
