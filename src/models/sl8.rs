use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoSpec {
    name: String,
    crew: String,
    dev_ops_tool_set: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentSpec {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub repo: RepoSpec,
    pub components: Vec<ComponentSpec>,
}

impl Config {
    pub fn new(name: &String, crew: &String, dev_ops_tool_set: &String) -> Self {
        Config {
            repo: RepoSpec {
                name: name.clone(),
                crew: crew.clone(),
                dev_ops_tool_set: dev_ops_tool_set.clone(),
            },
            components: Vec::new(),
        }
    }
}