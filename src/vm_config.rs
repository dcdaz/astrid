use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VMConfig {
    pub qemu_arch: String,
    pub boot: Option<String>,
    pub cdrom: Option<String>,
    pub drive: Option<String>,
    pub memory: Option<String>,
    pub cpu_type: Option<String>,
    pub cpu_cores: Option<String>,
    pub usb_devices: Option<String>,
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
        args.push("-enable-kvm".to_string());

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

        if self.cpu_type.is_some() {
            args.push("-cpu".to_string());
            args.push(self.cpu_type.as_ref().unwrap().into());
        }

        if self.cpu_cores.is_some() {
            args.push("-smp".to_string());
            args.push(self.cpu_cores.as_ref().unwrap().into());
        }

        if self.usb_devices.is_some() {
            args.push("-usb".to_string());
            self.usb_devices.clone().unwrap().split(",").into_iter().for_each(|device| {
                let usb_identifier = device.split(":").collect::<Vec<&str>>();
                args.push(
                    format!(
                        "-device usb-host,vendorid=0x{},productionid=0x{}",
                        usb_identifier[0],
                        usb_identifier[1]
                    )
                );
            });
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
