// use crate::folder_manager::{create_folder, write_to_file};

// pub fn process_by_leptess(image_path: String) -> Result<String, std::str::Utf8Error> {
//     let mut lt = leptess::LepTess::new(None, "eng").unwrap();
//     lt.set_image(image_path.to_string());
//     lt.get_utf8_text()
// }

// fn extract_by_leptess(folder_name: String, image_path: String) {
//     create_folder(&folder_name);

//     let output = process_by_leptess(image_path).unwrap();

//     let mut output_file_path = folder_name.clone();
//     output_file_path.push_str("/leptess_output.txt");

//     write_to_file(folder_name, path, content)
// }
