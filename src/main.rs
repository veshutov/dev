use clap::{arg, command, Command};

mod branch;

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
        .subcommand(
            Command::new("branch")
                .about("Creates new branch from trunk"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("module", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").unwrap();
            module::create_module(&path)
        }
        Some(("branch", _)) => {
            branch::create_new_branch_from_trunk()
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}