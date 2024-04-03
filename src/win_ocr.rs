use win_ocr::ocr;

use crate::folder_manager::{create_folder, write_to_file};

macro_rules! custom_debug {
    () => {
        // println!("Please Enter to proceed");

        // let mut buf = String::new();
        // io::stdin()
        //     .read_line(&mut buf)
        //     .expect("Failed to read line");
    };

    ($prompt:expr) => {{
            // println!("{}", $prompt);

            // custom_debug!();
    }};

    (2,$prompt:expr) => {{
        // println!("{:?}", $prompt);

        // custom_debug!("");
    }};
}

pub fn process_by_win_ocr(image_path: String) -> String {
    custom_debug!(" process_by_win_ocr started");
    let text: String = ocr(&image_path).unwrap();
    custom_debug!(" process_by_win_ocr finished");

    text
}

pub fn extract_by_win_ocr(folder_name: String, image_path: String) {

    create_folder(&folder_name);

    let text: String = process_by_win_ocr(image_path.clone());

    let mut output_file_path = folder_name.clone();
    output_file_path.push_str(&format!("/{}.txt",image_path).to_string());

    write_to_file(folder_name, &output_file_path, text.clone());

}
