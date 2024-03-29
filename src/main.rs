use std::fs::{self, File};
use std::io::prelude::*;

use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let files = get_files(&args.from);

    for file in files {
        let content = read_content(&file);

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&args.file)
            .unwrap();

        let file_content = read_content(&args.file);

        let new_content = content.replace("type", "export type").replace("interface", "export interface");

        new_content.lines().for_each(|line| {
            println!("{} in {}", line, &args.file);
            if !file_content.contains(line) {
                file.write_all(line.replace("export export", "export").as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            } else {
                // add some color to it
                x//println!("{} already exists in {}", line, &args.file);
            }
        })
    }
}

fn get_files(from: &str) -> Vec<String> {
    let mut files = vec![];
    let paths = std::fs::read_dir(from).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() && path.extension().unwrap() == "ts" {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}

fn read_content(file: &str) -> String {
    let mut file = File::open(file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    return content;
}
