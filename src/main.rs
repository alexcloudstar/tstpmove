use clap::Parser;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::utils::{
    get_files,
    create_file_in_folder,
    read_content,
    add_export_keyword
};
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
    let output_file_content =
        read_content(PathBuf::from(&args.to).join(&args.file).to_str().unwrap());

    let files = get_files(&args.from);
    for file_path in files {
        let content = read_content(&file_path);
        let blocks = get_block_of_code(&content);

        println!("Blocks: {:?}", blocks);
        content.lines().for_each(|line| {
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


fn get_block_of_code(content: &str) -> Vec<String> {
    let mut block_of_code = Vec::new();
    let mut is_block = false;

    for line in content.lines() {
        if line.contains("type") || line.contains("interface") {
            is_block = true;
        }

        if is_block {
            block_of_code.push(line.to_string());
        }

        if line.contains("}") {
            is_block = false;
        }
    }

    block_of_code
}


