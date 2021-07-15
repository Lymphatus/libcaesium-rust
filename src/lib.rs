mod png;
mod utils;
mod jpeg;

use std::error::Error;
use crate::utils::get_filetype;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct C_CSParameters {
    pub keep_metadata: bool,
    pub jpeg_quality: u32,
    pub png_level: u32,
    pub optimize: bool,
}

pub struct CSParameters {
    pub jpeg: jpeg::Parameters,
    pub png: png::Parameters,
    pub keep_metadata: bool,
    pub optimize: bool,
}

pub fn initialize_parameters() -> CSParameters
{
    let jpeg_parameters = jpeg::Parameters {
        quality: 80
    };

    let png_parameters = png::Parameters {
        oxipng: oxipng::Options::default(),
        level: 3
    };

    CSParameters {
        jpeg: jpeg_parameters,
        png: png_parameters,
        keep_metadata: false,
        optimize: false,
    }
}

#[no_mangle]
pub extern fn c_compress(input_path: *const c_char, output_path: *const c_char, params: C_CSParameters) -> bool {
    unsafe {
        let mut parameters = initialize_parameters();
        parameters.jpeg.quality = params.jpeg_quality;
        parameters.png.level = params.png_level - 1;
        parameters.optimize = params.optimize;
        parameters.keep_metadata = params.keep_metadata;

        compress(CStr::from_ptr(input_path).to_str().unwrap().to_string(),
                 CStr::from_ptr(output_path).to_str().unwrap().to_string(),
                 parameters)
            .unwrap();

        true
    }
}

pub fn compress(input_path: String, output_path: String, parameters: CSParameters) -> Result<(), Box<dyn Error>> {
    let file_type = get_filetype(&input_path);
    if parameters.jpeg.quality <= 0 || parameters.jpeg.quality > 100 {
        return Err("Invalid JPEG quality value")?;
    }

    if parameters.png.level > 6 {
        return Err("Invalid PNG quality value")?;
    }
    match file_type {
        utils::SupportedFileTypes::JPEG => {
            if parameters.optimize {
                unsafe {
                    jpeg::optimize(input_path, output_path, parameters)?;
                }
            } else {
                jpeg::compress(input_path, output_path, parameters)?;
            }
        }
        utils::SupportedFileTypes::PNG => {
            png::optimize(input_path, output_path, parameters)?;
        }
        _ => return Err("Unknown file type")?
    }

    Ok(())
}