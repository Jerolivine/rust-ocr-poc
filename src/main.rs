use std::{
    fs, io,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use screen_capture_tesseract_deneme::{
    folder_manager::{self, create_folder},
    // leptess::process_by_leptess,
    ocrs::{extract_by_ocrs, process_by_ocrs},
    tesseract::{extract_by_tesseract, process_by_tesseract},
    win_ocr::extract_by_win_ocr,
};
use screenshots::Screen;

macro_rules! custom_debug {
    () => {
        // println!("Please Enter to proceed");

        // let mut buf = String::new();
        // io::stdin()
        //     .read_line(&mut buf)
        //     .expect("Failed to read line");
    };

    ($prompt:expr) => {{
        println!("{}", $prompt);

        custom_debug!();
    }};

    (2,$prompt:expr) => {{
        println!("{:?}", $prompt);

        custom_debug!("");
    }};
}

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);

    // capture_screen();
    // for i in 0..10000 {
    //     let output = process_by_tesseract("image/1.png".to_string());
    //     thread::sleep(Duration::from_secs(1));
    //     // process_by_ocrs("image/1.png".to_string()).unwrap();
    // }

    let files = list_files_in_directory("image").unwrap();

    for file in files {
        println!("{:?}", file);

        // extract_by_tesseract("tesseract".to_string(), file.to_str().unwrap().to_string());
        // extract_by_ocrs("ocrs".to_string(), file.to_str().unwrap().to_string()).unwrap();
        // extract_by_win_ocr("win_ocr".to_string(), file.to_str().unwrap().to_string());
    }

    // let file_name = "image/issue-221-doc2.png".to_string();

    // extract_by_tesseract("tesseract".to_string(), file_name.clone());
    //Hello extract_by_ocrs("ocrs".to_string(),"image/1.png".to_string()).unwrap();

    // only windows
    // extract_by_win_ocr("win_ocr".to_string(), file_name.clone());

    // extract_by_leptess("leptess", "image/1.png".to_string());

    let _ = io::stdin().read_line(&mut buf);

    // custom_debug!("Program finished");
}

fn capture_screen() {
    create_folder("image");

    let screens = Screen::all().unwrap();

    let mut count = 0;

    for screen in screens {
        println!("capturer {screen:?}");

        count += 1;

        let image = screen.capture().unwrap();
        image.save(format!("image/{}.png", count)).unwrap();
    }
}

fn list_files_in_directory<P: AsRef<Path>>(directory: P) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}
