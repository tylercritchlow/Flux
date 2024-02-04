use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::Metadata;
use std::io::{BufRead, BufReader};
use std::{
    fs::{self, File},
    io::{Read, Write},
};

pub fn compress_files(files: Vec<String>) {
    let mut files_compressed = String::new();

    for file in files {
        if let file_path = file {
            let mut file_compressed = compress_single_file(&file_path);
            // prepend the file path to the compressed data
            file_compressed = format!("\n{}:\n\n{}", file_path, file_compressed);
            files_compressed.push_str(&file_compressed);
        }
    }
    let mut test_file = File::create("compressed.txt").expect("Failed to create test.txt file.");
    test_file
        .write_all(files_compressed.as_bytes())
        .expect("Failed to write to test.txt file.");
}

// commit file is the file that contains the compressed data, the name of the file should be the commit hash
pub fn decompress_and_write(commit_file: &str) {
    // read the commit file. it should be a string of the compressed data with a header that contains the file path
    // That looks like this: src/main.rs:\n[compressed data]
    // let mut commit_file = File::open(commit_file).expect("Failed to open commit file.");
    let metadata = fs::metadata(commit_file).expect("Failed to read metadata of commit file.");
    let buffer: Vec<u8> = vec![0; metadata.len() as usize];
    let buffer_clone = buffer.clone();

    let commit_file_contents = unsafe { String::from_utf8_unchecked(buffer) };

    let lines_vec: Vec<&str> = commit_file_contents.split(":\n").collect();

    let binding = lines_vec.clone().join(""); // binding prevents the temporary value from being dropped
    let linesbinding = binding.split("\n").collect::<Vec<_>>();
    // println!("{:?}", linesbinding);
    println!("{:?}", linesbinding.len());
    let lines = linesbinding
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    println!("{:?}", lines[0]);

    let decompressed = decompress_bytes(&buffer_clone);
    println!("{}", decompressed);
}

fn compress_single_file(file_path: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    let mut file = File::open(file_path).expect("File not found");
    let metadata = fs::metadata(file_path).expect("Unable to read metadata of file.");
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("Buffer overflow");

    encoder
        .write_all(&buffer)
        .expect("Unable to write to buffer");

    let buffer: Vec<u8> = encoder.finish().expect("Unable to finish compression");

    unsafe { String::from_utf8_unchecked(buffer) }
}

fn decompress_bytes(compressed_data: &[u8]) -> String {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed = String::new();
    decoder
        .read_to_string(&mut decompressed)
        .expect("Unable to decompress string");
    unsafe { String::from_utf8_unchecked(decompressed.into()) }
}
