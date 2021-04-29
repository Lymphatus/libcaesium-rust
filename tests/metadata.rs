use std::sync::Once;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use exif::{Tag, In, Field};

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        if fs::metadata(file).is_ok() {
            fs::remove_file(file).unwrap();
        }
    });
}

#[test]
fn compress_80_with_metadata() {
    let output = "tests/samples/output/compressed_80_metadata.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.quality = 80;
    pars.keep_metadata = true;
    caesium::compress(String::from("tests/samples/uncompressed.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let model = get_model_metadata(Path::new(output));
    assert_eq!(model.display_value().to_string(), "\"Canon EOS 2000D\"")
}

#[test]
fn optimize_with_metadata() {
    let output = "tests/samples/output/compressed_optimized_metadata.jpg";
    initialize(output);
    let mut pars = caesium::initialize_parameters();
    pars.jpeg.optimize = true;
    pars.keep_metadata = true;
    caesium::compress(String::from("tests/samples/uncompressed.jpg"), String::from(output), pars).unwrap();
    assert!(std::path::Path::new(output).exists());
    let model = get_model_metadata(Path::new(output));
    assert_eq!(model.display_value().to_string(), "\"Canon EOS 2000D\"")
}

fn get_model_metadata(path: &Path) -> Field {
    let file = File::open(path).unwrap();
    let exif = exif::Reader::new().read_from_container(
        &mut BufReader::new(&file)).unwrap();

    let f =  exif.get_field(Tag::Model, In::PRIMARY).unwrap();

    f.clone()
}