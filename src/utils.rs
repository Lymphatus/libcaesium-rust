use std::fs::File;
use std::io::Read;

pub enum SupportedFileTypes {
    JPEG,
    PNG,
    UNKN,
}

pub fn get_filetype(file_path: &String) -> SupportedFileTypes {
    let mut f = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return SupportedFileTypes::UNKN
    };
    let mut buffer = [0; 2];

    let jpeg_header = [0xFF as u8, 0xD8];
    let png_header = [0x89 as u8, 0x50];

    // read up to 2 bytes
    let read_result = f.read(&mut buffer);

    match read_result {
        Ok(rs) => rs,
        Err(_) => return SupportedFileTypes::UNKN
    };

    if buffer.iter().zip(jpeg_header.iter()).all(|(a, b)| a == b) {
        return SupportedFileTypes::JPEG;
    } else if buffer.iter().zip(png_header.iter()).all(|(a, b)| a == b) {
        return SupportedFileTypes::PNG;
    }

    SupportedFileTypes::UNKN
}