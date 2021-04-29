use std::path::PathBuf;
use oxipng::PngError;
use crate::CSParameters;

pub struct Parameters {
    pub oxipng: oxipng::Options
}

pub fn optimize(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), PngError> {
    let in_file = oxipng::InFile::Path(PathBuf::from(input_path));
    let out_file = oxipng::OutFile::Path(Some(PathBuf::from(output_path)));
    let mut oxipng_options = parameters.png.oxipng;

    if !parameters.keep_metadata {
        oxipng_options.strip = oxipng::Headers::Safe;
    }

    if parameters.quality >= 90 {
        oxipng_options.deflate = oxipng::Deflaters::Zopfli;
    } else {
        let preset = match parameters.quality {
            1..=39 => 0,
            40..=49 => 1,
            50..=59 => 2,
            60..=69 => 3,
            70..=79 => 5,
            80..=89 => 6,
            _ => 0
        };
        oxipng_options = oxipng::Options::from_preset(preset);
    }
    oxipng::optimize(&in_file, &out_file, &oxipng_options)
}