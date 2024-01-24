use std::{fs::{self, read_to_string, File}, io::{Read, Write}, path::Path};
use flate2::{Compress, Compression};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;


pub fn compress_files_added(files: Vec<Option<String>>) {

    for file in files {
        if let Some(file_path) = file {
            let data_to_be_compressed = read_to_string(&file_path);
            compress_single_file(&file_path);
        }
    }
}

// just public for testing
pub fn compress_single_file(file_path: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    //TODO: Compress a single file given the file path, and return String of the compressed file contents

    let mut file = File::open(file_path).expect("no file found");
    let metadata = fs::metadata(file_path).expect("Unable to read matadata");
    let mut buffer = vec![0; metadata.len() as usize]; 
    file.read(&mut buffer).expect("buffer overflow");

    encoder.write_all(&buffer).expect("Unable to write to buffer");

    let buffer: Vec<u8> = encoder.finish().expect("Unable to finish compression");

    String::from_utf8(buffer).expect("Invalid UTF-8")
}