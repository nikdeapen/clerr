# Issues

## 1. Allocation Performance

The `colored` crate's `ColoredString` type owns its string data. When building reports, this can lead to many small
allocations. This is unlikely to be a real performance issue since info & error reports should not be super common.
There seem to be optimizations that can be made within the crate to reduce allocations as well.

## 2. Testing

The crate lacks assertion-based tests. The existing tests only use `println!` for manual inspection and cannot catch
regressions in output formatting. Tests should strip ANSI escape codes and assert on the plain-text output.
