use std::io;

mod packages;
mod read_me;
mod ya_make;

pub fn create_module(parent_path: &String) {
    let module_name = read_module_name();

    let mut path = parent_path.to_owned();
    match path.chars().nth(path.len() - 1).unwrap() {
        '/' => path.push_str(&module_name),
        _ => path.push_str(&format!("/{}", module_name))
    };

    packages::create_main_folders(&path);
    packages::create_test_folders(&path);
    read_me::create_readme(&path);
    ya_make::create_ya_make(&path);
    ya_make::create_tests_ya_make(&path);

    print!("Created module {}", path)
}

fn read_module_name() -> String {
    println!("Enter module name:");

    let mut module_name = String::new();

    io::stdin()
        .read_line(&mut module_name)
        .expect("Failed to module name");

    return module_name.trim().to_owned();
}
