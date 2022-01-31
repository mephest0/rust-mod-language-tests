use std::fs::read_to_string;

pub fn load_file(file_name: &str) -> String {
    let text = read_to_string(file_name)
        .expect("Error reading file");

    text
}
