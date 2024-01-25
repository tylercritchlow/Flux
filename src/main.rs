mod parse_flux;
mod compression_and_decompression;

use crate::compression_and_decompression::*;

use clap::App;
use std::{fs, vec};
use std::io::Write;

fn main() {
    let matches = App::new("Flux SCM")
        .version("0.0.1")
        .author("Tyler Critchlow")
        .about("Version control system.")
        .subcommand(
            App::new("init")
                .about("Initialize Flux")
                .version("1.0")
                .author("Tyler Critchlow"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            // check if .flux directory exists
            if fs::metadata(".flux").is_ok() {
                println!("Reinitializing Flux repository");

                // Delete the .flux directory
                fs::remove_dir_all(".flux").expect("Failed to remove .flux directory.");
            }

            fs::create_dir(".flux").expect("Failed to create .flux directory.");

            // Create a config file in the .flux directory
            let config_path = ".flux/config.toml";
            let mut config_file =
                fs::File::create(config_path).expect("Failed to create config.toml file.");
            config_file
                .write_all(b"key = value")
                .expect("Failed to write to config.toml file.");

            // create a branches directory
            // In flux, branches are stored as directories in the branches directory
            // Each branch directory contains a file called HEAD that contains the hash of the commit that the branch is currently pointing to

            fs::create_dir(".flux/branches").expect("Failed to create branches directory.");
            fs::create_dir(".flux/branches/main").expect("Failed to create main branch directory.");

            //TODO: write the hash of the initial commit to the main-HEAD file
            let mut main_head_file = fs::File::create(".flux/branches/main/main-HEAD")
                .expect("Failed to create main-HEAD file.");
            main_head_file.write_all(b"# this file contains the commit that this branch points to, i.e. the commit that will appear in the files if you check it out. #\n\nHASH == [placeholder]").expect("Failed to write to main-HEAD file.");

            // Create a commits directory
            //TODO: Create a function to generate a commit file in the commits directory, with the commit hash as the file name
            fs::create_dir(".flux/branches/main/commits")
                .expect("Failed to create commits directory.");

            println!("Initialized repository in the .flux directory.");
        }
        _ => println!("Please provide a valid command. Use --help for more information."),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_flux::*;

    #[test]
    fn test_get_flux_head() {
        let branch = "main";
        let cwd = "/home/tyler/RustProjects/flux";
        let expected = "/home/tyler/RustProjects/flux/.flux/branches/main/main-HEAD";
        let result = get_flux_head(branch.to_string(), cwd.to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_flux_head_hash() {
        let branch = "main";
        let cwd = "/home/tyler/RustProjects/flux";
        // Assuming the file contains "abc123"
        let expected = "abc123";
        let result = get_flux_head_hash(branch, cwd);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_flux_branch() {
        let branch = "main";
        let cwd = "/home/tyler/RustProjects/flux";
        let expected = "/home/tyler/RustProjects/flux/.flux/branches/main";
        let result = get_flux_branch(branch, cwd);
        assert_eq!(result, expected);
    }
}
