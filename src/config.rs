use std::{fs};

use log::warn;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub admins: Vec<u64>,
    pub admin_chat: i64,
    pub rate_limit_secs: u32,
    pub tokens: Token,
    pub dirs: Dirs,
    pub triggers: Triggers,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct Triggers {
    pub first_trigger: Vec<String>,
    pub second_trigger: Vec<String>,
    pub hiss: TriggersHiss,
    pub special: TriggersSpecial,
}

#[derive(Deserialize)]
pub struct TriggersHiss {
    pub first_trigger: Vec<String>,
}

#[derive(Deserialize)]
pub struct TriggersSpecial {
    pub first_trigger: Vec<String>,
}

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
    pub test_token: String,
}

#[derive(Deserialize)]
pub struct Dirs {
    pub opossum: String,
    pub hiss: String,
    pub special: String,
}

pub fn get_token(cfg: Config, testing: bool) -> String {
    return match testing {
        true => cfg.tokens.test_token,
        false => cfg.tokens.token,
    };
}

pub fn get_cfg() -> Result<Config, ()> {
    let data = fs::read_to_string("/home/misty/bots_rs/tg_opossum/src/config.toml");
    match data {
        Ok(toml_data) => {
            return match toml::from_str(&toml_data) {
                Ok(cfg) => Ok(cfg),
                Err(e) => {
                    log::error!("Couldn't parse `config.rs`: {}", e);
                    Err(())
                }
            };
        }
        Err(e) => {
            warn!("`config.toml` not found.");
            return Err(());
        }
    }
}
