use std::fs::File;
use std::io::prelude::*;

pub fn create_readme(module_name: &String) {
    let mut readme = File::create(format!("{}/README.md", module_name))
        .expect("Error encountered while creating ya.make!");
    readme.write_all(b"#TODO module description")
        .expect("Error while writing to README.md");
}