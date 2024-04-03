use std::{thread, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_tesseract::{Args, Image};
use screen_capture_tesseract_deneme::{
    ocrs::process_by_ocrs, tesseract::process_by_tesseract, win_ocr::process_by_win_ocr,
};

fn tesseract_benchmark() {
    // let img = Image::from_path("image/1.png").unwrap();

    // let my_args = Args::default();

    // // string output
    // let image_to_string: Result<String, rusty_tesseract::TessError> =
    //     rusty_tesseract::image_to_string(&img, &my_args);

    process_by_tesseract("image/1.png".to_string());
}

fn ocrs_benchmark() {
    process_by_ocrs("image/1.png".to_string()).unwrap();
}

fn win_ocr_benchmark() {
    process_by_win_ocr("image/1.png".to_string());
}

fn test_benchmark() {
    thread::sleep(Duration::from_secs(1));
}

// Define the benchmark function
fn criterion_tesseract_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("OCR Benchmarks");
     group.sample_size(10);
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("tesseract_benchmark", |b| {
        b.iter(|| {
            tesseract_benchmark();
        })
    });

    // group.bench_function("ocrs_benchmark", |b| {
    //     b.iter(|| {
    //         ocrs_benchmark();
    //     })
    // });

    // group.bench_function("win_ocr_benchmark", |b| {
    //     b.iter(|| {
    //         win_ocr_benchmark();
    //     })
    // });

    group.finish();
}

fn criterion_ocrs_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("OCR Benchmarks");
     group.sample_size(10);
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("ocrs_benchmark", |b| {
        b.iter(|| {
            ocrs_benchmark();
        })
    });

    // group.bench_function("win_ocr_benchmark", |b| {
    //     b.iter(|| {
    //         win_ocr_benchmark();
    //     })
    // });

    group.finish();
}

fn criterion_win_ocr_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("OCR Benchmarks");
     group.sample_size(10);
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("win_ocr_benchmark", |b| {
        b.iter(|| {
            win_ocr_benchmark();
        })
    });

    group.finish();
}

fn criterion_test_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("OCR Benchmarks");
     group.sample_size(10);
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("test_benchmark", |b| {
        b.iter(|| {
            test_benchmark();
        })
    });

    group.finish();
}


criterion_group!(benches, criterion_tesseract_benchmark);
// criterion_group!(benches,criterion_ocrs_benchmark);
// criterion_group!(benches,criterion_win_ocr_benchmark);
// criterion_group!(benches,criterion_test_benchmark);
// criterion_group!(benches,criterion_tesseract_benchmark,criterion_ocrs_benchmark,criterion_win_ocr_benchmark);
criterion_main!(benches);
