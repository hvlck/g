// std
use std::fs::{read_to_string, write};
use std::path::Path;

// crates
use clap::{App, AppSettings, Arg, SubCommand};
use g_c::parse;
use walkdir::WalkDir;

// local
use gcli::Configuration;

mod new;
use new::scaffold;

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("build")
                .about("Build a G project.")
                .arg(
                    Arg::with_name("release")
                        .short("r")
                        .help("Build a G project for release."),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .help("The directory to write the final output to")
                        .takes_value(true)
                        .default_value("./out/"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Build and run a G project.")
                .arg(
                    Arg::with_name("open")
                        .short("o")
                        .long("open")
                        .help("Open project in-browser automatically"),
                ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Build and run a G project, and recompile on file save.")
                .arg(
                    Arg::with_name("open")
                        .long("open")
                        .short("o")
                        .help("Open project in-browser automatically."),
                ),
        )
        .subcommand(
            SubCommand::with_name("doc")
                .about("Generate documentation.")
                .arg(
                    Arg::with_name("open")
                        .short("o")
                        .long("open")
                        .help("Open documentation in-browser automatically."),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new G project.")
                .arg(
                    Arg::with_name("name")
                        .short("n")
                        .long("new")
                        .takes_value(true)
                        .required(false)
                        .help("The name of the new project."),
                ),
        )
        .subcommand(SubCommand::with_name("fmt").about("Format your code."))
        .settings(&[AppSettings::ColoredHelp])
        .bin_name("gc")
        .get_matches();

    if let Some(value) = app.subcommand_matches("build") {
    } else if let Some(value) = app.subcommand_matches("run") {
    } else if let Some(value) = app.subcommand_matches("serve") {
    } else if let Some(value) = app.subcommand_matches("doc") {
    } else if let Some(v) = app.subcommand_matches("new") {
        let name = v.value_of("name").unwrap();
        match scaffold(&Path::new("."), name) {
            Ok(()) => {
                println!("Successfully created project {}", name);
            },
            Err(err) => {
                println!("Something went wrong: {}", err);
            }
        };
    } else if let Some(value) = app.subcommand_matches("fmt") {
    }
}
