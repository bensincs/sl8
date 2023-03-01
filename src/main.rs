mod models;
mod generate;
mod init;
mod project;
mod cli;

use std::{
    io::{Error}, fs::File
};
use generate::generate;
use init::init;
use project::project;
use cli::cli;

use models::sl8::{Config};
use serde_json::Error as SerdeError;

fn get_sl8_config(file_path: &str) -> Result<Config, Error> {
    let sl8_file_result = File::open(file_path);

    let sl8_file = match sl8_file_result {
        Ok(file) => file,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                format!("Could not find the sl8 config file at: {}", file_path),
            ));
        }
    };

    let sl8_config_result: Result<Config, SerdeError> = serde_json::from_reader(sl8_file);

    let sl8_config: Config = match sl8_config_result {
        Ok(spec) => spec,
        Err(error) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Could not parse the sl8 config file at: {} with error: {}", file_path, error),
            ));
        }
    };  return Ok(sl8_config);
}

fn main() -> Result<(), Error> {
    let matches = cli().get_matches();

    let sl8_file_path = matches.get_one::<String>("config").unwrap();

    match matches.subcommand() {
        Some(("generate", generate_matches)) => {
            let sl8_config = get_sl8_config(&sl8_file_path).unwrap();

            generate( &sl8_config, &generate_matches).unwrap();
        }

        Some(("init", init_matches)) => {
            init(&sl8_file_path, &init_matches).unwrap();
        }

        Some(("project", project_matches)) => {
            let mut sl8_config = get_sl8_config(&sl8_file_path).unwrap();
            project(&mut sl8_config, &sl8_file_path, &project_matches).unwrap();
        }

        _ => {
            println!("No subcommand was used");
        }
    }

    Ok(())
}

