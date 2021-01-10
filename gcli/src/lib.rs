// std

// crates
use serde::{Deserialize, Serialize};

// local

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub output: String
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            output: String::from("./out/")
        }
    }
}
