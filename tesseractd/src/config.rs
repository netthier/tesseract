use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub cri: CriConfig,
}

impl Config {
    pub fn init(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let mut f = File::open(path)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf)?)
    }
}

#[derive(Deserialize)]
pub struct CriConfig {
    pub endpoint: String,
}