use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let (query, path) = (&arguments[1], &arguments[2]);

    dbg!(query, path);
}
