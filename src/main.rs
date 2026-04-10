#![windows_subsystem = "windows"]

use clipboard_win::{formats, set_clipboard};
use std::env;

fn main() {
    let mut quote = false;
    let mut paths: Vec<String> = Vec::new();

    for arg in env::args().skip(1) {
        if arg == "--quote" {
            quote = true;
        } else {
            paths.push(arg);
        }
    }

    if paths.is_empty() {
        eprintln!("Usage: copy-paths-to-clipboard [--quote] <path1> [path2] ...");
        std::process::exit(1);
    }

    let result: Vec<String> = paths
        .iter()
        .map(|p| {
            let forward = p.replace('\\', "/");
            if quote {
                format!("\"{}\"", forward)
            } else {
                forward
            }
        })
        .collect();

    let text = result.join("\n");
    set_clipboard(formats::Unicode, &text).expect("Failed to set clipboard");
    println!("{text}");
}
