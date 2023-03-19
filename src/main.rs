use clap::{arg, command, Command};

mod arc;
mod module;

fn main() {
    let matches = command!() // requires `cargo` feature
        .subcommand_required(true)
        .subcommand(
            Command::new("module")
                .visible_alias("m")
                .about("Creates new module")
                .arg(arg!([path] "Path to parent folder")
                    .default_value("./")),
        )
        .subcommand(
            Command::new("branch")
                .visible_alias("b")
                .about("Creates new branch from trunk")
                .arg(arg!([name] "Branch name")),
        )
        .subcommand(
            Command::new("rebase")
                .visible_alias("r")
                .about("Rebases current branch on trunk"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("module", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").unwrap();
            module::create_module(&path)
        }
        Some(("branch", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            arc::create_new_branch_from_trunk(name)
        }
        Some(("rebase", _)) => {
            arc::rebase_current_branch_on_trunk()
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}