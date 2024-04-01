use std::{
    fs::{self, File},
    path::PathBuf,
};

pub fn get_files(folder: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        let path = path.unwrap().path();

        if path.to_str().unwrap().contains("node_modules") {
            continue;
        }

        if path.is_dir() {
            files.append(&mut get_files(path.to_str().unwrap()));
        } else {
            if path.to_str().unwrap().ends_with(".ts") {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    files
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
        if line.contains("type") || line.contains("interface") {
            is_block = true;
            block_of_code.push(String::new());
        }

        if line.contains("const") || line.contains("import") {
            is_block = false;
        }

        if is_block && line.contains(": {") {
            is_inner_block = true;
        }

        if is_block || is_inner_block {
            if block_of_code.len() <= idx {
                block_of_code.push(String::new());
            }

            block_of_code[idx].push_str(line);
        }

        if line.contains("}") && !is_inner_block {
            is_block = false;
            idx += 1;
        }
    }

    return block_of_code.join("\n\n");
}
