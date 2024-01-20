// logic to parse and mutate the flux directory, basically the brain of Flux. This is the first step in commits, checking out branches, and other operations.

use std::{fs, io::Read};

pub fn get_flux_branch(branch: &str) -> String {
    let mut branch_path = String::from(".flux/branches/{}");
    branch_path.push_str(branch);
    branch_path
}

pub fn get_flux_head(branch: &str) -> String {
    // This should get the file path to the commit hash file
    let mut head_path = String::from(".flux/branches/{}/{}-HEAD");
    head_path.push_str(branch);
    head_path
}

pub fn get_flux_head_hash(branch: &str) -> String {
    // Copilot generated this code, it's not quite right
    let head_path = get_flux_head(branch);
    let mut head_file = fs::File::open(head_path).expect("Failed to open HEAD file.");
    let mut head_hash = String::new();
    head_file
        .read_to_string(&mut head_hash)
        .expect("Failed to read HEAD file.");
    head_hash
}