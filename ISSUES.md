# Issues

## 1. Allocation Performance

The `colored` crate's `ColoredString` type owns its string data. When building reports, this can lead to many small
allocations. This is unlikely to be a real performance issue since info & error reports should not be super common.
There seem to be optimizations that can be made within the crate to reduce allocations as well.
