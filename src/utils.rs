use std::{
    fs::{self, File},
    path::PathBuf,
};

pub fn get_files(from: &str) -> Vec<String> {
    fs::read_dir(from)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.to_str().unwrap().contains(".ts") {
                return path.to_str().map(ToString::to_string);
            }
            None
        })
        .collect()
}

pub fn read_content(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn add_export_keyword(line: &str) -> String {
    if line.contains("export export") {
        line.replace("export export", "export")
    } else if !line.contains("export") && (line.contains("type") || line.contains("interface")) {
        line.replace("type", "export type")
            .replace("interface", "export interface")
    } else {
        line.to_string()
    }
}

pub fn create_file_in_folder(folder: &str, file_name: &str) -> std::io::Result<File> {
    let mut path = PathBuf::from(folder);
    path.push(file_name);

    if let Some(dir_path) = path.parent() {
        fs::create_dir_all(dir_path)?;
    }

    File::create(&path)
}

pub fn get_block_of_code(content: &str) -> String {
    let mut block_of_code: Vec<String> = Vec::new();
    let mut is_block = false;
    let mut is_inner_block = false;
    let mut idx = 0;


    for line in content.lines() {
        println!("Line: {}", line);
        if line.contains("type") || line.contains("interface") {
            is_block = true;
            block_of_code.push(String::new());
        }

        if line.contains("const") {
            is_block = false;
        }

        if line.contains(": {") {
            is_inner_block = true;
        }

        if is_block {
            block_of_code[idx].push_str(line);
        }

        if line.contains("}") && !is_inner_block  {
            is_block = false;
            idx += 1;
        }

    }

    return block_of_code.join("\n\n");
}

