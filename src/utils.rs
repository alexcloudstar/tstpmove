use std::{fs::{self, File}, path::PathBuf};

pub fn get_files(from: &str) -> Vec<String> {
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
