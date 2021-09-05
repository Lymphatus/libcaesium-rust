use std::fs::File;
use std::io::Read;

pub enum SupportedFileTypes {
    Jpeg,
    Png,
    Unkn,
}

pub fn get_filetype(file_path: &str) -> SupportedFileTypes {
    let mut f = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return SupportedFileTypes::Unkn
    };
    let mut buffer = [0; 2];

    let jpeg_header = [0xFF_u8, 0xD8];
    let png_header = [0x89_u8, 0x50];

    // read up to 2 bytes
    let read_result = f.read(&mut buffer);

    match read_result {
        Ok(rs) => rs,
        Err(_) => return SupportedFileTypes::Unkn
    };

    if buffer.iter().zip(jpeg_header.iter()).all(|(a, b)| a == b) {
        return SupportedFileTypes::Jpeg;
    } else if buffer.iter().zip(png_header.iter()).all(|(a, b)| a == b) {
        return SupportedFileTypes::Png;
    }

    SupportedFileTypes::Unkn
}