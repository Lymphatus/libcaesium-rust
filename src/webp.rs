use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::ops::Deref;
use webp;
use crate::CSParameters;

pub struct Parameters {
    pub quality: u32,
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), io::Error>
{
    let mut input_file = File::open(input_path)?;

    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;
    let decoder = webp::Decoder::new(&input_data);
    let input_webp = match decoder.decode() {
        Some(img) => img,
        None => return Err(io::Error::new(io::ErrorKind::Other, "WebP decode failed!"))
    };
    let input_image = input_webp.to_image();

    let encoder = match webp::Encoder::from_image(&input_image) {
        Ok(encoder) => encoder,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e))
    };

    let mut output_file = File::create(output_path)?;
    if parameters.optimize {
        output_file.write_all(encoder.encode_lossless().deref())?;
    } else {
        output_file.write_all(encoder.encode(parameters.webp.quality as f32).deref())?;
    }
    Ok(())
}