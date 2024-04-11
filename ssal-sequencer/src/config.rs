use std::{fs, path::Path};

use ssal_core::{
    error::{Error, WrapError},
    serde::{Deserialize, Serialize},
    toml,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "ssal_core::serde")]
pub struct Config {
    pub ssal_url: String,
    pub rollup_id: String,
    pub chain_url: String,
    pub wallet_private_key: String,
    pub is_local_deployment: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ssal_url: "http://127.0.0.1:3000".to_string(),
            rollup_id: "1".to_string(),
            chain_url: "http://127.0.0.1:8545".to_string(),
            wallet_private_key: "wallet_private_key".to_string(),
            is_local_deployment: true,
        }
    }
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file_string =
            fs::read_to_string(path).wrap(format!("Failed to read the file to String"))?;
        let config: Self = toml::from_str(&file_string)
            .wrap("Failed to parse the config file into type: Config")?;
        Ok(config)
    }
}
