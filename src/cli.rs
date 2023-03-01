use clap::{Command, arg};



pub fn cli() -> Command {
    Command::new("sl8")
        .about("A tool to generate template repositories for your projects")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .args(&[
            arg!(--config <FILE> "a required files for the configuration and no short").default_value("./.sl8.json"),
        ])
        .subcommand(Command::new("init")
            .short_flag('I')
            .long_flag("init")
            .args(&[
                arg!(-n --name  <NAME> "a name for the repo").required(true),
                arg!(-c --crew <CREW> "a name for the crew").required(true),
                arg!(-d --devops  <DEVOPS> "the type of dev ops tool set to use").default_value("azure-pipelines"),
            ])
            .about("Initializes the template sl8 config file")
        )
        .subcommand(Command::new("project")
            .short_flag('P')
            .long_flag("proj")
            .about("Add a new project to the sl8 config file")
            .subcommand_required(true)
            .subcommand(Command::new("add")
                    .args(&[
                        arg!(<NAME> "a name for the project").required(true)
                    ])
                    .about("Add a new project to the sl8 config file")
            )
            .subcommand(Command::new("clear")
                    .about("Clears all projects from the sl8 config file")
            )
        )
        .subcommand(Command::new("generate")
            .short_flag('G')
            .long_flag("gen")
            .about("Generates the template repository")
        )
}