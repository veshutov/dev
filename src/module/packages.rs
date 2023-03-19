use std::fs;

pub fn create_main_folders(module_name: &String) {
    let main_path = format!("{}/src/main/java/ru/yandex/plus", module_name);

    fs::create_dir_all(main_path)
        .expect("Error encountered while creating packages!");
}

pub fn create_test_folders(module_name: &String) {
    let tests_path = format!("{}/src/test/java/ru/yandex/plus", module_name);

    fs::create_dir_all(tests_path)
        .expect("Error encountered while creating packages!");
}

