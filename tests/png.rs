use caesium;
use std::sync::Once;
use std::fs;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        if fs::metadata(file).is_ok() {
            fs::remove_file(file).unwrap();
        }
    });
}

pub fn cleanup(file: &str) {
    if fs::metadata(file).is_ok() {
        fs::remove_file(file).unwrap();
    }
}

#[test]
fn standard_compress_png() {
    let output = "tests/samples/output/compressed.png";
    initialize(output);
    caesium::compress(String::from("tests/samples/uncompressed.png"),
                      String::from(output),
                      caesium::initialize_parameters())
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}

#[test]
fn zopfli_compress_png() {
    let output = "tests/samples/output/optimized.png";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.png.level = 3;
    params.optimize = true;
    caesium::compress(String::from("tests/samples/uncompressed.png"),
                      String::from(output),
                      params)
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    cleanup(output)
}
