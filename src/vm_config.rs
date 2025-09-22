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

    pub fn get_command_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.enable_kvm {
            args.push("-enable-kvm".to_string());
        }

        if self.boot.is_some() {
            args.push("-boot".to_string());
            args.push(self.boot.as_ref().unwrap().into());
        }

        if self.cdrom.is_some() {
            args.push("-cdrom".to_string());
            args.push(self.cdrom.as_ref().unwrap().into());
        }

        if self.drive.is_some() {
            args.push("-drive".to_string());
            args.push(format!("file={}", self.drive.as_ref().unwrap()));
        }

        if self.memory.is_some() {
            args.push("-m".to_string());
            args.push(self.memory.as_ref().unwrap().into());
        }

        if self.cpu.is_some() {
            args.push("-cpu".to_string());
            args.push(self.cpu.as_ref().unwrap().into());
        }

        if self.vga.is_some() {
            args.push("-vga".to_string());
            args.push(self.vga.as_ref().unwrap().into());
        }

        if self.display.is_some() {
            args.push("-display".to_string());
            args.push(self.display.as_ref().unwrap().into());
        }

        args
    }
}