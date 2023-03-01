use std::{fs::File, io::Write};

use clap::ArgMatches;

use crate::models::{sl8::{Config, ComponentSpec}};

pub fn project(sl8_config: &mut Config, sl8_file_path: &String, project_matches: &ArgMatches) -> Result<(), ()> {
    match project_matches.subcommand() {
        Some(("add", project_add_matches)) => {
            let name = project_add_matches.get_one::<String>("NAME").unwrap();
            sl8_config.components.push(ComponentSpec {
                name: String::from(name),
            });
        }
        Some(("clear", _)) => {
            sl8_config.components.clear();
        }
        _ => {
            println!("No subcommand was used");
        }
    }

    let mut file = File::create(&sl8_file_path).unwrap();
    let config_json = serde_json::to_string_pretty(&sl8_config).unwrap();

    file.write_all(config_json.as_bytes()).unwrap();

    return Ok(());
}