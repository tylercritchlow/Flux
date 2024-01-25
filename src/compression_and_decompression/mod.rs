use std::{fs::{self, read_to_string, File}, io::{Read, Write}, path::Path};
use flate2::{Compress, Compression};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;


pub fn compress_files(files: Vec<Option<String>>) {
    for mut file in files {
        if let Some(file_path) = file.take() {
            compress_single_file(&file_path);
            let mut file_compressed = compress_single_file(&file_path);
            // prepend the file path to the compressed data
            file_compressed = format!("{}:\n{}", file_path, file_compressed);
            // write the compressed data to a test.txt file
            let mut test_file = File::create("test.txt").expect("Failed to create test.txt file.");
            test_file.write_all(file_compressed.as_bytes()).expect("Failed to write to test.txt file.");
        }
    }
}

pub fn decompress_files()
fn compress_single_file(file_path: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    let mut file = File::open(file_path).expect("File not found");
    let metadata = fs::metadata(file_path).expect("Unable to read metadata of file.");
    let mut buffer = vec![0; metadata.len() as usize]; 
    file.read(&mut buffer).expect("Buffer overflow");

    encoder.write_all(&buffer).expect("Unable to write to buffer");

    let buffer: Vec<u8> = encoder.finish().expect("Unable to finish compression");

    unsafe { String::from_utf8_unchecked(buffer) }
}

fn decompress_string(compressed_string: &str) -> String {
    let mut decoder = GzDecoder::new(compressed_string.as_bytes());
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).expect("Unable to decompress string");
    decompressed
}