use std::{fs::{self, read_to_string, File}, io::{Read, Write}, path::Path};
use flate2::{Compress, Compression};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;


pub fn compress_files_added(files: Vec<Option<String>>) {

    for file in files {
        if let Some(file_path) = file {
\           compress_single_file(&file.unwrap().as_str());
            
        }
    }
}

pub fn compress_single_file(file_path: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    let mut file = File::open(file_path).expect("no file found");
    let metadata = fs::metadata(file_path).expect("Unable to read matadata");
    let mut buffer = vec![0; metadata.len() as usize]; 
    file.read(&mut buffer).expect("buffer overflow");

    encoder.write_all(&buffer).expect("Unable to write to buffer");

    let buffer: Vec<u8> = encoder.finish().expect("Unable to finish compression");

    unsafe { String::from_utf8_unchecked(buffer) }
}

pub fn decompress_string(compressed_string: &str) -> String {
    let mut decoder = GzDecoder::new(compressed_string.as_bytes());
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).expect("Unable to decompress string");
    decompressed
}