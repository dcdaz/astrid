use std::process::Command;

use super::configuration::Configuration;
use super::vm_config::VMConfig;

pub fn run_astrid(args: Vec<String>) {
    let config = Configuration::new();

    match args[1].as_str() {
        "-c" => create_image(config, args),
        "-l" => list_vms(config),
        "-r" => run_vm(config, args),
        "-h" => print_help(),
        _ => print_help(),
    }
}

pub fn print_help() {
    println!(
        r#"
        astrid works with the following args:
            -c ImageName ImageSize  -> creates images
            -l                      -> list all vm configs
            -r ConfigName           -> run a vm config
            -h                      -> prints this help menu
        "#
    )
}

fn create_image(config: Configuration, args: Vec<String>) {
    let image_path = format!(
        "{}/{}",
        config.vms_path,
        &args[2]
    );
    // Currently it creates Qcow2 images, not sure if add more types
    Command::new("qemu-img")
        .args(["create", "-f", "qcow2", image_path.as_str(), &args[3]])
        .spawn()
        .expect("Failed to execute command");
}

fn list_vms(config: Configuration) {
    let vm_configs = std::fs::read_dir(config.vms_path.clone()).unwrap();
        vm_configs
            .filter(|c| c.as_ref().unwrap().file_name().display().to_string().contains("yml"))
            .for_each(|c| println!("{}", c.unwrap().file_name().display()));
}

fn run_vm(config: Configuration, args: Vec<String>) {
    let vm_config_name = args[2].clone();
        let vm_config_path = format!(
            "{}/{}.yml",
            config.vms_path,
            vm_config_name
        );
        let vm_config = VMConfig::new(vm_config_path.as_str());

        let qemu_command= format!("qemu-system-{}",vm_config.qemu_arch);
        let mut command = Command::new(qemu_command);
        command.args(vm_config.get_command_args());
        command.spawn().expect("Failed to execute command");
}
