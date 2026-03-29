# Issues

## Allocation Performance

The `colored` crate's `ColoredString` type owns its string data. When building reports, this can lead to many small
allocations. This is unlikely to be a real performance issue since info & error reports should not be present in any
high performance code. There are optimizations that can be made but this would come at a high cost of usability.

## Testing

The crate lacks assertion-based tests. The existing tests only use `println!` for manual inspection and cannot
automatically catch regressions in output formatting. Automated regression testing is preferred, but it is not only
difficult to test visuals but there is also issues with `ColoredString` comparison on different platforms.

## TokenInfo Alignment

`TokenInfo` rendering uses `position` and `token_len` to generate spaces and carets for alignment. This assumes each
character in `line_text` occupies exactly one display column, which breaks for multibyte UTF-8 characters, wide
characters (CJK), and tabs.
