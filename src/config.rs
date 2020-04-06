use dirs::{config_dir, data_dir};
use log::{error, trace};
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub color: bool,
    pub storage_location: PathBuf,
}

fn config_root() -> PathBuf {
    let loc = config_dir().or(data_dir()).unwrap().join("rstd");
    if !loc.exists() {
        create_dir_all(&loc).expect(&format!("Could not create the config dir {:?}", loc));
    }
    loc
}

impl ConfigFile {
    pub fn initialize() -> Self {
        let default = ConfigFile::default();
        let loc = config_root().join("/config.yml");
        // All these fancy pants safe error handling, pattern matches, so that we
        // may achieve a fancier looking IF
        match File::open(loc) {
            Ok(file) => {
                trace!("Config file found!");
                match from_reader(file) {
                    // Found and parsed.
                    Ok(cfg) => {
                        trace!("Config loaded sucessfully");
                        cfg
                    }
                    // found & could't be parsed. Gotta be loud here.
                    Err(e) => {
                        error!("Could not parse config. (Error: {})", e);
                        error!("Failing back to default:\n{:?}", default);
                        default
                    }
                }
            }
            // not found can't fail to parse, use default
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
            storage_location: config_root().join("storage.db"),
        }
    }
}
