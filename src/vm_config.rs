use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VMConfig {
    pub qemu_arch: String,
    pub enable_kvm: bool,
    pub boot: Option<String>,
    pub cdrom: Option<String>,
    pub drive: Option<String>,
    pub memory: Option<String>,
    pub cpu: Option<String>,
    pub vga: Option<String>,
    pub display: Option<String>,
}

impl VMConfig {
    pub fn new(config_name: &str) -> Self {
        let config = Config::builder()
            .add_source(File::with_name(config_name))
            .build();
        config.unwrap().try_deserialize::<VMConfig>().unwrap()
    }
}