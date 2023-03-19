use std::process::Command;

pub fn create_new_branch_from_trunk(branch_name: &String) {
    assert!(Command::new("arc")
        .arg("pull")
        .arg("trunk")
        .spawn()
        .expect("Failed to pull trunk")
        .wait()
        .expect("Error while waiting trunk pull")
        .success());

    assert!(Command::new("arc")
        .arg("checkout")
        .arg("trunk")
        .spawn()
        .expect("Failed to checkout trunk")
        .wait()
        .expect("Error while waiting trunk checkout")
        .success());

    assert!(Command::new("arc")
        .arg("checkout")
        .arg(format!("-b{}", branch_name))
        .spawn()
        .expect("Failed to checkout new branch")
        .wait()
        .expect("Error while waiting for branch checkout")
        .success());
}

pub fn rebase_current_branch_on_trunk() {
    assert!(Command::new("arc")
        .arg("pull")
        .arg("trunk")
        .spawn()
        .expect("Failed to pull trunk")
        .wait()
        .expect("Error while waiting for trunk pull")
        .success());

    assert!(Command::new("arc")
        .arg("rebase")
        .arg("trunk")
        .spawn()
        .expect("Failed to rebase on trunk")
        .wait()
        .expect("Error while waiting for rebase")
        .success());
}
