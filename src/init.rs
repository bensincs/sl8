use std::{fs::File, io::Write};

use clap::ArgMatches;

use crate::models::sl8::Config;

pub fn init(sl8_file_path: &str, init_matches: &ArgMatches) -> Result<(), ()> {
    let name = init_matches.get_one::<String>("name").unwrap();
    let crew = init_matches.get_one::<String>("crew").unwrap();
    let dev_ops_tool_set = init_matches.get_one::<String>("devops").unwrap();

    let config = Config::new(&name, &crew, &dev_ops_tool_set);
    let config_json = serde_json::to_string_pretty(&config).unwrap();
    let mut file = File::create(&sl8_file_path).unwrap();
    file.write_all(config_json.as_bytes()).unwrap();

    return Ok(());
}