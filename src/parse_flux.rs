// logic to parse and mutate the flux directory, basically the brain of Flux. This is the first step in commits, checking out branches, and other operations.

// cwd is the current working directory, which is the directory that the user is in when they run flux commands

use std::{fs};
use std::io::BufRead;

pub fn get_flux_branch(branch: &str, cwd: &str) -> String {
    let branch_path = format!("{}/.flux/branches/{}", cwd, branch);
    branch_path
}

pub fn get_flux_head(branch: String, cwd: String) -> String {
    // This should get the file path to the commit hash file
    let head_path = format!("{}/.flux/branches/{}/{}-HEAD", cwd, branch, branch);
    head_path
}

pub fn get_flux_head_hash(branch: &str, cwd: &str) -> String {
    // This should get the commit hash from the commit hash file
    let head_path = format!("{}/.flux/branches/{}/{}-HEAD", cwd, branch, branch);
    let head_file = fs::File::open(head_path).expect("Failed to open HEAD file.");
    let mut head_hash = String::new();
    // parse the file to get the hash. skip lines that start with #
    let mut reader = std::io::BufReader::new(head_file);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).expect("Failed to read line.");
        // if the line starts with #, or is empty, skip it
        if line.starts_with("#") || line.is_empty() {
            continue;
        } else {
            // if the line doesn't start with # write it to the line variable and break the loop
            line = reader.lines().next().unwrap().unwrap();
            head_hash = line;
            break;
        }
    }
    head_hash
}
