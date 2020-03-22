use serde::Deserialize;

use colored::Colorize;
use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub http_port: i32,
    pub https_port: i32,

    pub serve_root: String
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            http_port: 80,
            https_port: 0,

            serve_root: "./html".to_string()
        }
    }
}

pub fn load_settings() -> Settings {
    let file = File::open("./config.json");

    if file.is_err() {
        println!(
            "[{}] {:8} Cannot location './config.json' using default",
            "*".blue(),
            "Settings".cyan().bold()
        );

        return Settings::default();
    }

    let file = file.unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    serde_json::from_reader(reader).unwrap_or_else(|_| Settings::default())
}