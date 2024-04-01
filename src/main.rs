use clap::Parser;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::utils::{add_export_keyword, create_file_in_folder, get_files, read_content, get_block_of_code};
mod utils;

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

    let mut output_file = create_file_in_folder(&args.to, &args.file)?;
    let output_file_content: String =
        read_content(PathBuf::from(&args.to).join(&args.file).to_str().unwrap());

    let files = get_files(&args.from);
    for file_path in files {
        let content = read_content(&file_path);
        let blocks = get_block_of_code(&content);

        blocks.lines().for_each(|line| {
            if !output_file_content.contains(line) {
                let new_line = add_export_keyword(line);
                if !new_line.trim().is_empty() {
                    writeln!(output_file, "{}", new_line).unwrap();
                }
            }
        });
    }

    Ok(())
}

