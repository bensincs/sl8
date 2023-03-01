use std::{
    fs::{create_dir_all, File},
    io::{Error, Write},
    path::Path,
};

use clap::ArgMatches;
use handlebars::Handlebars;
use include_dir::{include_dir, Dir};

use crate::models::sl8::Config;

static TEMPLATE_REPO_DIR: Dir<'_> = include_dir!(".repo-template");
static TEMPLATE_COMPONENT_DIR: Dir<'_> = include_dir!(".component-template");
static TEMPLATE_PIPELINE_DIR: Dir<'_> = include_dir!(".pipeline-template");

static PARTIALS_DIR: Dir<'_> = include_dir!(".partials");

pub fn generate(sl8_config: &Config, _: &ArgMatches) -> Result<(), Error> {


    let mut repo_hbs = Handlebars::new();
    let mut comp_hbs = Handlebars::new();
    let mut pipeline_hbs = Handlebars::new();


    let glob = "**/*.hbs";

    PARTIALS_DIR
        .find(glob)
        .unwrap()
        .for_each(|entry| {
            let file = entry.as_file().unwrap();
            let file_name = file.path().file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".hbs") {
                let file_name_no_ext = file_name.replace(".hbs", "");
                repo_hbs.register_partial(file_name_no_ext.as_str(), String::from_utf8_lossy(file.contents()).to_string())
                .unwrap();
                comp_hbs.register_partial(format!("{}", file_name_no_ext).as_str(), String::from_utf8_lossy(file.contents()).to_string()).unwrap();
                pipeline_hbs.register_partial(format!("{}", file_name_no_ext).as_str(), String::from_utf8_lossy(file.contents()).to_string()).unwrap();
            }
        });

    TEMPLATE_REPO_DIR.find(glob).unwrap().for_each(|entry| {
        let file = entry.as_file().unwrap();
        let file_name = file.path().file_name().unwrap().to_str().unwrap();
        let file_path = file.path().to_str().unwrap();
        if file_name.ends_with(".hbs") {
            repo_hbs.register_template_string(
                &file_path,
                String::from_utf8_lossy(file.contents()).to_string(),
            )
            .unwrap();
        }
    });

    TEMPLATE_COMPONENT_DIR.find(glob).unwrap().for_each(|entry| {
        let file = entry.as_file().unwrap();
        let file_name = file.path().file_name().unwrap().to_str().unwrap();
        let file_path = file.path().to_str().unwrap();
        if file_name.ends_with(".hbs") {
            comp_hbs.register_template_string(
                &file_path,
                String::from_utf8_lossy(file.contents()).to_string(),
            )
            .unwrap();
        }
    });

    TEMPLATE_PIPELINE_DIR.find(glob).unwrap().for_each(|entry| {
        let file = entry.as_file().unwrap();
        let file_name = file.path().file_name().unwrap().to_str().unwrap();
        let file_path = file.path().to_str().unwrap();
        if file_name.ends_with(".hbs") {
            pipeline_hbs.register_template_string(
                &file_path,
                String::from_utf8_lossy(file.contents()).to_string(),
            )
            .unwrap();
        }
    });

    repo_hbs.get_templates().keys().filter(|f| f.contains(".hbs")).for_each(|k| {
        let file_path_str = &k.replace(".hbs", "");
        let file_path = Path::new(file_path_str);
        let file_parent_path_str = file_path.parent().unwrap().to_str().unwrap();
        let content = repo_hbs.render(&k, sl8_config).unwrap();
        if content.len() != 0 {
            create_dir_all(file_parent_path_str).unwrap();
            let mut output_file = File::create(&file_path).unwrap();
            output_file.write_all(content.as_bytes()).unwrap();
        }
    });

    for component in sl8_config.components.iter() {
        comp_hbs.get_templates().keys().filter(|f| f.contains(".hbs")).for_each(|k| {
            let file_path_str = format!("./components/{}/{}",component.name, k.replace(".hbs", ""));
            let file_path = Path::new(&file_path_str);
            let file_parent_path_str = file_path.parent().unwrap().to_str().unwrap();
            let content = comp_hbs.render(&k, &component).unwrap();
            if content.len() != 0 {
                create_dir_all(file_parent_path_str).unwrap();
                let mut output_file = File::create(&file_path).unwrap();
                output_file.write_all(content.as_bytes()).unwrap();
            }
        });

        pipeline_hbs.get_templates().keys().filter(|f| f.contains(".hbs")).for_each(|k| {
            let file_path_str = format!("./.devops/pipelines/{}.{}",component.name, k.replace(".hbs", ""));
            let file_path = Path::new(&file_path_str);
            let file_parent_path_str = file_path.parent().unwrap().to_str().unwrap();
            let content = pipeline_hbs.render(&k, &component).unwrap();
            if content.len() != 0 {
                create_dir_all(file_parent_path_str).unwrap();
                let mut output_file = File::create(&file_path).unwrap();
                output_file.write_all(content.as_bytes()).unwrap();
            }
        });
    }

    Ok(())
}
