# Changes
## 1.0.0-alpha.0
Initial release.
## 1.0.0-alpha.1
- Process skip tags in a much more robust way.
  - Fix bug where some blocks would only parse correctly with certain combinations of skip tags.
- Improve error types.
- Make node input list parsing use `Result` instead of panicking on error.
## 1.0.0-alpha.2
- Add `build_file` function.
- Add `NodeType` enum and make `Node` use it.
- Add `FILE_START` constant with magic numbers and specification version.
- Fix specification version constants.
