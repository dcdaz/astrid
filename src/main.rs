use std::process::exit;

mod astrid;
mod configuration;
mod vm_config;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() > 4 {
        args.remove(0);
        eprintln!("Error during parsing arguments [{}]", args.join(","));
        astrid::print_help();
        exit(1);
    }
    astrid::run_astrid(args);
}