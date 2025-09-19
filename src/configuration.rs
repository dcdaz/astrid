use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub vms_path: String
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration { 
            vms_path: format!(
                "{}/{}",
                dirs::config_dir().unwrap().as_os_str().to_str().unwrap(),
                "astrid.yml"
            )
        }
    }
}

impl Configuration {
    pub fn new() -> Self {
        match dirs::config_dir() {
            Some(path) => {
                let full_path = format!(
                    "{}/{}",
                    path.as_os_str().to_str().unwrap(),
                    "astrid.yml"
                );

                Self::get_config_from_file(&full_path).unwrap()
            },
            None => Self::default()
        }
    }

    fn get_config_from_file(file_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
        .add_source(File::with_name(file_path))
        .build()?;
        config.try_deserialize::<Configuration>()
    }
}