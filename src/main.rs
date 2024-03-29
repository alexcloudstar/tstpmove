use clap::Parser;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    from: String,

    #[arg(long)]
    to: String,

    #[arg(long)]
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Create the output file once outside the loop
    let mut output_file = create_file_in_folder(&args.to, &args.file)?;
    let output_file_content = read_content(PathBuf::from(&args.to).join(&args.file).to_str().unwrap());

    let files = get_files(&args.from);
    for file_path in files {
        let content = read_content(&file_path);
        content.lines().for_each(|line| {
            if !output_file_content.contains(line) {
                let new_line = add_export_keyword(line);
                if !new_line.trim().is_empty() { // Avoid writing empty lines
                    writeln!(output_file, "{}", new_line).unwrap();
                }
            }
        });
    }

    Ok(())
}

fn get_files(from: &str) -> Vec<String> {
    fs::read_dir(from)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension()? == "ts" {
                return path.to_str().map(ToString::to_string);
            }
            None
        })
        .collect()
}

fn read_content(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn add_export_keyword(line: &str) -> String {
    if line.contains("export export") {
        line.replace("export export", "export")
    } else if !line.contains("export") && (line.contains("type") || line.contains("interface")) {
        line.replace("type", "export type").replace("interface", "export interface")
    } else {
        line.to_string()
    }
}

fn create_file_in_folder(folder: &str, file_name: &str) -> std::io::Result<File> {
    let mut path = PathBuf::from(folder);
    path.push(file_name);

    if let Some(dir_path) = path.parent() {
        fs::create_dir_all(dir_path)?;
    }

    File::create(&path)
}

