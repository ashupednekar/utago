use config::{Config, ConfigError, File, FileFormat};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Lyrics {
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub lyrics: Lyrics,
}

impl Settings {
    pub fn new() -> Result<Settings, ConfigError> {
        Config::builder()
            .add_source(File::new("../config.toml", FileFormat::Toml))
            .build()?
            .try_deserialize()
    }
}

lazy_static! {
    pub static ref settings: Settings = Settings::new().expect("improperly configured");
}
