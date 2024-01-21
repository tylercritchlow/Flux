// logic to parse and mutate the flux directory, basically the brain of Flux. This is the first step in commits, checking out branches, and other operations.

// cwd is the current working directory, which is the directory that the user is in when they run flux commands

use std::{fs, io::Read};

pub fn get_flux_branch(branch: &str, cwd: &str) -> String {
    let mut branch_path = format!("{}/.flux/branches/{}", cwd, branch);
    branch_path
}

pub fn get_flux_head(branch: String, cwd: String) -> String {
    // This should get the file path to the commit hash file
    let mut head_path = format!("{}/.flux/branches/{}/{}-HEAD", cwd, branch, branch);
    head_path
}

// pub fn get_flux_head_hash(branch: &str, cwd: &str) -> String {
//     // Copilot generated this code, it's not quite right
//     let head_path = get_flux_head(branch, cwd);
//     let mut head_file = fs::File::open(head_path).expect("Failed to open HEAD file.");
//     let mut head_hash = String::new();
//     head_file
//         .read_to_string(&mut head_hash)
//         .expect("Failed to read HEAD file.");
//     head_hash
// }