use std::process::Command;

mod configuration;
mod vm_config;

fn main() {
    let args: Vec<_> = std::env::args().collect(); // get all arguments passed to app
    if args.len() > 3 {
        panic!("Only one arg");
    }

    let config = configuration::Configuration::new();

    if args[1] == "-l" {
        let vm_configs = std::fs::read_dir(config.vms_path.clone()).unwrap();
        vm_configs
            .filter(|c| c.as_ref().unwrap().file_name().display().to_string().contains("yml"))
            .for_each(|c| println!("{}", c.unwrap().file_name().display()));
    }

    if args[1] == "-n" {
        let vm_config_name = args[2].clone();
        let vm_config_path = format!(
            "{}/{}.yml",
            config.vms_path,
            vm_config_name
        );
        let vm_config = vm_config::VMConfig::new(vm_config_path.as_str());
        println!("{:#?}", vm_config);

        let qemu_command= format!("qemu-system-{}",vm_config.qemu_arch);
        let mut command = std::process::Command::new(qemu_command);
        if vm_config.enable_kvm {
            command.arg("-enable-kvm");
        }
        add_args(&mut command, vm_config.boot, "-boot");
        add_args(&mut command, vm_config.drive, "-drive");
        add_args(&mut command, vm_config.memory, "-m");
        add_args(&mut command, vm_config.cpu, "-cpu");
        add_args(&mut command, vm_config.vga, "-vga");
        add_args(&mut command, vm_config.display, "-display");
        command.spawn().expect("Failed to execute command");
    }
    
}

fn add_args(command: &mut Command, possible_arg: Option<String>, arg_name: &str) {
    match possible_arg {
        Some(arg) => command.args([arg_name, arg.as_str()]),
        None => command
    };
}