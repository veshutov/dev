use clap::{arg, command, Command};

mod module;

fn main() {
    let matches = command!() // requires `cargo` feature
        .subcommand_required(true)
        .subcommand(
            Command::new("module")
                .about("Creates new module")
                .arg(arg!([path] "Path to parent folder")
                    .default_value("./")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("module", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").unwrap();
            module::create_module(&path)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}