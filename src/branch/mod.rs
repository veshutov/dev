use std::io;
use std::process::Command;

pub fn create_new_branch_from_trunk() {
    let branch_name = read_branch_name();

    Command::new("arc")
        .arg("pull")
        .arg("trunk")
        .spawn()
        .expect("Failed to pull trunk")
        .wait()
        .expect("Error while waiting trunk pull");

    Command::new("arc")
        .arg("checkout")
        .arg("trunk")
        .spawn()
        .expect("Failed to checkout trunk")
        .wait()
        .expect("Error while waiting trunk checkout");

    Command::new("arc")
        .arg("checkout")
        .arg(format!("-b{}", branch_name))
        .spawn()
        .expect("Failed to checkout new branch")
        .wait()
        .expect("Error while waiting new branch checkout");
}

fn read_branch_name() -> String {
    println!("Enter branch name:");

    let mut branch_name = String::new();

    io::stdin()
        .read_line(&mut branch_name)
        .expect("Failed to read branch name");

    return branch_name.trim().to_string();
}