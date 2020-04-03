use log::{trace, error};
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::env;
use std::fs::File;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    color: bool,
    storage_location: String,
}

fn config_root() -> String {
    let def_loc = String::from("~/.config");
    env::var("XDG_USER_DIR").unwrap_or(def_loc) + "/rstd"
}

impl ConfigFile {
    pub fn initialize() -> Self {
        let default = ConfigFile::default();
        let loc = config_root() + "/config.yml";
        match File::open(loc) {
            Ok(file) => {
                trace!("Config file found!");
                match from_reader(file) {
                    Ok(cfg) => {
                        trace!("Config loaded sucessfully");
                        cfg
                    }
                    Err(e) => {
                        error!("Could not parse config. (Error: {})", e);
                        error!("Failing back to default:\n{:?}", default);
                        default
                    }
                }
            }
            Err(e) => {
                trace!("Config file not found. Error: {}", e);
                trace!("Loading default: {:?}", default);
                default
            }
        }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            color: false,
            storage_location: config_root() + "/storage.yml",
        }
    }
}
