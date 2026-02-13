#![windows_subsystem = "windows"]

use clipboard_win::{formats, set_clipboard};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: copy-paths-to-clipboard <path1> [path2] ...");
        std::process::exit(1);
    }

    let result: Vec<String> = args
        .iter()
        .map(|p| {
            let forward = p.replace('\\', "/");
            format!("\"{}\"", forward)
        })
        .collect();

    let text = result.join("\n");
    set_clipboard(formats::Unicode, &text).expect("Failed to set clipboard");
    println!("{text}");
}
