use clap::App;
use std::fs;
use std::io::Write;

fn main() {
    let matches = App::new("Flux SCM")
        .version("0.0.1")
        .author("Tyler Critchlo")
        .about("Simple version control system.")
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
                println!("Reinitialized existing Flux repository in .flux");
                return;
            }

            fs::create_dir(".flux").expect("Failed to create .flux directory.");

            // Create a config file in the .flux directory
            let config_path = ".flux/config.toml";
            
            // broken right now, need to fix
            let mut config_file = fs::File::create(config_path).expect("Failed to create config.toml file.");
            config_file.write_all(b"key = value").expect("Failed to write to config.toml file.");

            println!("Initialized empty Flux repository in .flux");
        }
        _ => println!("Please provide a valid command. Use --help for more information."),
    }
}
