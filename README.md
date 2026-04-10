# copy-paths-to-clipboard

A Total Commander helper that copies selected paths to the clipboard with forward slashes, optionally double-quoted.

This complements Total Commander's built-in `cm_CopyFullNamesToClip` and `cm_CopyNamesToClip` commands, which copy paths with backslashes. Backslash paths are needed in file dialogs, but forward slashes are more suitable for use in scripts where backslashes are escape characters.

See https://ghisler.ch/board/viewtopic.php?t=52797.

## Total Commander Setup

Add custom commands via **Configuration → Options → Misc. → Redefine Hotkeys**.
Suggested keymap:

| Hotkey | Action |
|---|---|
| <kbd>\`</kbd> | `cm_CopyFullNamesToClip` (built-in, backslashes) |
| <kbd>Alt+\`</kbd> | `cm_CopyNamesToClip` (built-in, backslashes) |
| <kbd>Ctrl+\`</kbd> | `copy-paths-to-clipboard` (forward slashes) |
| <kbd>Ctrl+Shift+\`</kbd> | `copy-paths-to-clipboard --quote` (forward slashes, quoted) |

### Copy full paths with forward slashes (<kbd>Ctrl+\`</kbd>)

- **Command:** `path/to/copy-paths-to-clipboard`
- **Parameters:** `%P%S`

This copies the full paths of selected files to the clipboard:

```
C:/Projects/my app/config.toml
C:/Projects/my app/src/main.rs
```

### Copy quoted full paths with forward slashes (<kbd>Ctrl+Shift+\`</kbd>)

- **Command:** `path/to/copy-paths-to-clipboard`
- **Parameters:** `--quote %P%S`

This copies the full paths of selected files to the clipboard, double-quoted:

```
"C:/Projects/my app/config.toml"
"C:/Projects/my app/src/main.rs"
```

## Build

```sh
cargo build --release
```

The binary is at `target/release/copy-paths-to-clipboard`.

## Tip: Uppercase Drive Letters

By default Total Commander gives lowercase drive letters (e.g. `c:/`).
To get the conventional uppercase (`C:/`), add the following to your `wincmd.ini`:

```ini
[Configuration]
DrivesShowUpcase=1
DrivesExportUpcase=1
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
