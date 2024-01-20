use clap::App;
use std::fs;
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
                .author("Your Name"),
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

            // write the hash of the initial commit to the main-HEAD file
            let mut main_head_file = fs::File::create(".flux/branches/main/main-HEAD").expect("Failed to create main-HEAD file.");
            main_head_file.write_all(b"# this file contains the commit that this branch points to, i.e. the commit that will appear in the files if you check it out. #\n\nHASH == [placeholder]").expect("Failed to write to main-HEAD file.");

            println!("Initialized empty Flux repository in the .flux directory.");
        }
        _ => println!("Please provide a valid command. Use --help for more information."),
    }
}
