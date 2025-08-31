use clap::Parser;
use std::fs;
use std::process::{self, Command};
use std::io::Write;

#[derive(Parser)]
#[command(name = "cct")]
#[command(about = "Copy file content to clipboard")]
#[command(version = "1.0")]
struct Args {
    file: String,
}

fn copy_to_clipboard(content: &str) -> Result<(), String> {
    let mut child = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|err| format!("Failed to start wl-copy: {}. Make sure wl-clipboard is installed.", err))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(content.as_bytes())
            .map_err(|err| format!("Failed to write to wl-copy: {}", err))?;
        drop(stdin);
    }

    let status = child.wait()
        .map_err(|err| format!("Failed to wait for wl-copy: {}", err))?;

    if status.success() {
        Ok(())
    } else {
        Err("wl-copy command failed".to_string())
    }
}

fn main() {
    let args = Args::parse();

    // Read file content
    let content = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", args.file, err);
            process::exit(1);
        }
    };

    // Copy to clipboard using wl-copy
    if let Err(err) = copy_to_clipboard(&content) {
        eprintln!("Error copying to clipboard: {}", err);
        process::exit(1);
    }

    println!("File '{}' copied to clipboard", args.file);
}