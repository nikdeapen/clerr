# Issues

## Performance

- Reduce the many small allocations from `ColoredString` owning its data.

## Rendering

- Use `position.saturating_sub(1)` in the `TokenInfo` conversion so out-of-contract input can't wrap in release builds.
- Expand tabs in `TokenInfo::line_text` to spaces before rendering so caret alignment survives tab-indented lines.
- Collapse adjacent fragments that share a style; `Code` emits four separate color spans for `error[E001]: `.
- Drop the trailing space from `TokenInfo`'s blank separator rows, which end with `" | "` where `rustc` emits `  |`.
- Skip entries that convert to an empty `Vec<ColoredString>`; `Properties::default()` renders a stray blank line.
- Honor `f.width()`, `f.fill()`, and `f.align()` in the `Display` impls, padding by plain text width since
  `Formatter::pad` would count the escape bytes.

## API

- Compare and hash `Entry` by concatenated text, not per fragment; `["ab"]` and `["a", "b"]` render alike but differ.
- Forward `Report`'s `Debug` to `Display`; returning one from `main` prints a `ColoredString` struct dump.
- Decide whether `Report`'s `Error` impl is propagation-only; nesting it duplicates the `error[id]: ` prefix.
- Add a plain text entry, likely a `Text` type, so a one-line note doesn't need `vec!["...".normal()]`.

## Color Detection

- `colored` keys its tty check off stdout but reports print to stderr, so redirecting either stream miscolors output.

## Terminal Backgrounds

- Drop the `bright_white()` foreground from `Code`'s message; it is invisible on light backgrounds.
- Add light-background variants for `Severity::Warning` and `Severity::Info`; both are low contrast on white.
- Add a theme opt-in such as a `CLERR_THEME` env var; background detection is not reliable.

## Packaging

- Add `keywords` and `categories` to `Cargo.toml` for crates.io search.
- Add `rust-version = "1.85"` to `Cargo.toml`, the version edition 2024 requires.
- Add `exclude` to `Cargo.toml`; `.idea`, `.github`, and `.gitignore` ship in the published tarball.
