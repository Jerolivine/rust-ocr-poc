use crate::folder_manager::{create_folder, write_to_file};

use rten::ops::concat;
use rusty_tesseract::{Args, Image};
use std::{fs, rc::Rc};

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

pub fn extract_by_tesseract(folder_name: String, image_path: String) {
    let output = process_by_tesseract(image_path.clone());

    custom_debug!(" rusty_tesseract::image_to_string after");

    custom_debug!(2, output);
    custom_debug!(output.clone());

    custom_debug!(" image/tesseract_output.txt write");

    let mut output_file_path = folder_name.clone();
    output_file_path.push_str(&format!("/{}.txt",image_path).to_string());

    write_to_file(folder_name.to_string(), &output_file_path, output.clone());

    custom_debug!(output.clone());
}

pub fn process_by_tesseract(image_path: String) -> String {
    custom_debug!("get_image_text started");

    let img = Image::from_path(image_path).unwrap();

    let my_args = Args::default();

    custom_debug!(" rusty_tesseract::image_to_string started");

    // string output
    let image_to_string: Result<String, rusty_tesseract::TessError> =
        rusty_tesseract::image_to_string(&img, &my_args);

        custom_debug!(" rusty_tesseract::image_to_string finished");

    let output = match image_to_string {
        Ok(output) => output,
        Err(err) => {
            eprintln!("{:?}", err);
            String::new()
        }
    };

    output
}
