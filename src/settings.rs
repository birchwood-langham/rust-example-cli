use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    name: String,
    password: String,
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
