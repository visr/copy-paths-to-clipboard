# copy-paths-to-clipboard

A Total Commander helper that copies selected paths to the clipboard, double-quoted and with forward slashes.

## Total Commander Setup

Add two custom commands via **Configuration → Options → Misc. → Redefine Hotkeys**.
For example, you could bind them to <kbd>\`</kbd> and <kbd>Alt+\`</kbd>:

### Copy full paths (e.g. <kbd>\`</kbd>)

- **Command:** `path/to/copy-paths-to-clipboard`
- **Parameters:** `%P%S`

This copies the full paths of selected files to the clipboard:

```
"C:/Projects/my app/config.toml"
"C:/Projects/my app/src/main.rs"
```

### Copy filenames only (e.g. <kbd>Alt+\`</kbd>)

- **Command:** `path/to/copy-paths-to-clipboard`
- **Parameters:** `%S`

This copies just the filenames of selected files to the clipboard:

```
"config.toml"
"main.rs"
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
