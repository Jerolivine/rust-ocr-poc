use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use ocrs::{OcrEngine, OcrEngineParams, TextLine};
use rten::Model;
use rten_imageio::read_image;
use rten_tensor::prelude::*;

use std::io;

use crate::folder_manager::write_to_file;

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

struct Args {
    image: String,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut values = VecDeque::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => values.push_back(val.string()?),
            Long("help") => {
                println!(
                    "Usage: {bin_name} <image>",
                    bin_name = parser.bin_name().unwrap_or("hello_ocrs")
                );
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    let image = values.pop_front().ok_or("missing `image` arg")?;

    Ok(Args { image })
}

/// Read a file from a path that is relative to the crate root.
fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    fs::read(abs_path)
}

// cargo run --release --example hello_ocr rust-book.jpg
pub fn extract_by_ocrs(folder_name: String, image_path: String) -> Result<(), Box<dyn Error>> {
    custom_debug!("recognize_text completed");

    let mut text = String::new();

    let line_texts = process_by_ocrs(image_path.clone())?;

    for line in line_texts
        .iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
    {
        text.push_str(&line.to_string());
        println!("{}", line);
    }

    let mut output_file_path = folder_name.clone();
    output_file_path.push_str(&format!("/{}.txt",image_path).to_string());

    write_to_file(folder_name, &output_file_path, text.clone());

    Ok(())
}

pub fn process_by_ocrs(image_path: String) -> Result<Vec<Option<TextLine>>, Box<dyn Error>> {
    custom_debug!("Program Started");

    // Use the `download-models.sh` script to download the models.
    let detection_model_data = read_file("models/text-detection.rten")?;
    let rec_model_data = read_file("models/text-recognition.rten")?;

    custom_debug!("Models read");

    let detection_model = Model::load(&detection_model_data)?;
    let recognition_model = Model::load(&rec_model_data)?;

    custom_debug!("Models loaded");

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    custom_debug!("Engine created");

    // Read image using image-rs library and convert to a
    // (channels, height, width) tensor with f32 values in [0, 1].
    let image = read_image(&image_path)?;

    custom_debug!("read_image completed");

    // Apply standard image pre-processing expected by this library (convert
    // to greyscale, map range to [-0.5, 0.5]).
    let ocr_input = engine.prepare_input(image.view())?;

    custom_debug!("prepare_input completed");

    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // Get oriented bounding boxes of text words in input image.
    let word_rects = engine.detect_words(&ocr_input)?;

    custom_debug!("detect_words completed");

    // Group words into lines. Each line is represented by a list of word
    // bounding boxes.
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    custom_debug!("find_text_lines completed");

    // Recognize the characters in each line.
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    Ok(line_texts)
}
