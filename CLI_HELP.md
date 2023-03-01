# Command-Line Help for `sl8`

This document contains the help content for the `sl8` command-line program.

**Command Overview:**

* [`sl8`↴](#sl8)
* [`sl8 init`↴](#sl8-init)
* [`sl8 project`↴](#sl8-project)
* [`sl8 project add`↴](#sl8-project-add)
* [`sl8 project clear`↴](#sl8-project-clear)
* [`sl8 generate`↴](#sl8-generate)

## `sl8`

A tool to generate template repositories for your projects

**Usage:** `sl8 [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `init` — Initializes the template sl8 config file
* `project` — Add a new project to the sl8 config file
* `generate` — Generates the template repository

###### **Options:**

* `--config <FILE>` — a required files for the configuration and no short

  Default value: `./.sl8.json`



## `sl8 init`

Initializes the template sl8 config file

**Usage:** `sl8 init [OPTIONS] --name <NAME> --crew <CREW>`

###### **Options:**

* `-n`, `--name <NAME>` — a name for the repo
* `-c`, `--crew <CREW>` — a name for the crew
* `-d`, `--devops <DEVOPS>` — the type of dev ops tool set to use

  Default value: `azure-pipelines`



## `sl8 project`

Add a new project to the sl8 config file

**Usage:** `sl8 project <COMMAND>`

###### **Subcommands:**

* `add` — Add a new project to the sl8 config file
* `clear` — Clears all projects from the sl8 config file



## `sl8 project add`

Add a new project to the sl8 config file

**Usage:** `sl8 project add <NAME>`

###### **Arguments:**

* `<NAME>` — a name for the project



## `sl8 project clear`

Clears all projects from the sl8 config file

**Usage:** `sl8 project clear`



## `sl8 generate`

Generates the template repository

**Usage:** `sl8 generate`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
