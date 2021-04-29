mod png;
mod utils;
mod jpeg;
use std::error::Error;
use crate::utils::get_filetype;

pub struct CSParameters {
    pub jpeg: jpeg::Parameters,
    pub png: png::Parameters,
    pub keep_metadata: bool,
    pub quality: u32
}

pub fn initialize_parameters() -> CSParameters
{
    let jpeg_parameters = jpeg::Parameters {
        optimize: false
    };

    let png_parameters = png::Parameters {
        oxipng: oxipng::Options::default()
    };

    CSParameters {
        jpeg: jpeg_parameters,
        png: png_parameters,
        keep_metadata: false,
        quality: 80,
    }
}

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn print_str(s: *const c_char) {
    unsafe {
        let raw = CStr::from_ptr(s);
            if let Ok(msg) = raw.to_str() {
                println!("print str: {}", msg);
            }
        }
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), Box<dyn Error>> {
    let file_type = get_filetype(&input_path);
    if parameters.quality <= 0 || parameters.quality > 100 {
        return Err("Invalid quality value")?
    }
    match file_type {
        utils::SupportedFileTypes::JPEG => {
            if parameters.jpeg.optimize {
                unsafe {
                    jpeg::optimize(input_path, output_path, parameters)?;
                }
            } else {
                jpeg::compress(input_path, output_path, parameters)?;
            }
        },
        utils::SupportedFileTypes::PNG => {
            png::optimize(input_path, output_path, parameters)?;
        },
        _ => return Err("Unknown file type")?
    }

    Ok(())
}