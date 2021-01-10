// std
use std::fs::{create_dir, write};
use std::path::Path;

// crates
use toml::to_string_pretty;

// local
use gcli::Configuration;

static INDEX: &'static str = include_str!("../static/index.html");

static STYLING: &'static str = include_str!("../static/index.css");

fn generate_index(name: &str) -> Result<(), std::io::Error> {
    let i = INDEX.replace("{{ title }}", name);
    write(Path::new(&format!("./{}/index", name)), i)
}

fn generate_styling(name: &str) -> Result<(), std::io::Error> {
    write(Path::new(&format!("./{}/index.css", name)), STYLING)
}

fn generate_default_config(name: &str) -> Result<(), std::io::Error> {
    write(
        Path::new(&format!("./{}/g.toml", name)),
        to_string_pretty(&Configuration {
            ..Default::default()
        })
        .unwrap(),
    )
}

pub fn scaffold(p: &Path, name: &str) -> Result<(), std::io::Error> {
    let path = p.join(name);

    if path.is_dir() == true {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "The given directory already exists.",
        ));
    } else {
        create_dir(path.clone()).unwrap();
    }
    generate_index(name)?;
    generate_styling(name)?;
    generate_default_config(name)?;

    Ok(())
}
