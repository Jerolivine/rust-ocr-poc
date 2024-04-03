use std::fs;

pub fn create_folder(path: &str) {
    fs::create_dir(path).unwrap_or_default();
}

pub fn write_to_file(folder_name:String, path: &str, content: String) {

    create_folder(&folder_name);

    match fs::write(path, content.clone()) {
        Ok(_) => println!("Data written to file successfully."),
        Err(e) => println!("Failed to write to file: {}", e),
    }
}
