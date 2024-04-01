use clap::Parser;
use std::{io::prelude::*, time::Instant};
use std::path::PathBuf;

use crate::utils::{
    add_export_keyword, create_file_in_folder, get_block_of_code, get_files, read_content,
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
    let start = Instant::now();


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

    let elapsed = start.elapsed();
    println!("✨ Done, in {:.2} seconds, all types have been moved.", elapsed.as_secs_f32());

    Ok(())
}
