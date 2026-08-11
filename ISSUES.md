# Issues

## Allocation Performance

The `colored` crate's `ColoredString` type owns its string data. When building reports, this can lead to many small
allocations. This is unlikely to be a real performance issue since info & error reports should not be present in any
high performance code. There are optimizations that can be made but this would come at a high cost of usability.

## Rendering

- Use `position.saturating_sub(1)` in the `TokenInfo` conversion so out-of-contract input can't wrap in release builds.
- Expand tabs in `TokenInfo::line_text` to spaces before rendering so caret alignment survives tab-indented lines.

## Color Detection

- `colored` keys its tty check off stdout but reports print to stderr, so redirecting either stream miscolors output.
