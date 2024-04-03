fn process_by_paddleocr() {
    let mut p = paddleocr::Ppocr::new(
        PathBuf::from(".../PaddleOCR-json.exe"), // path to binary
        Default::default(), // language config_path, default `zh_CN`
    )
    .unwrap(); // initialize
    
    let now = std::time::Instant::now(); // benchmark
    {
        // OCR files
        println!("{}", p.ocr(Path::new(".../test1.png").into()).unwrap());
    }
    
}
