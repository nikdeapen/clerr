# Issues

## Allocation Performance

The `colored` crate's `ColoredString` type owns its string data. When building reports, this can lead to many small
allocations. This is unlikely to be a real performance issue since info & error reports should not be present in any
high performance code. There are optimizations that can be made but this would come at a high cost of usability.
