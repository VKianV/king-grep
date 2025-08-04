use std::{env, fs};

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let (query, file_path) = (&arguments[1], &arguments[2]);

    dbg!(query, file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    dbg!(contents);
}
