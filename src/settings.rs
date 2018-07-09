use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub name: String,
    pub password: String,
    pub database: Database,
    pub service: Service,
}

impl Settings {
    pub fn new(config_file: Option<String>) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // first merge the default file with the default settings
        s.merge(File::with_name("default.toml"))?;

        match config_file {
            Some(file_path) => {
                s.merge(File::with_name(&file_path).required(false))?;
                s.try_into()
            },
            None => s.try_into()
        }
    }
}
