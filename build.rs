use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};

fn main() {
    let path = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("src/path_to_enlightenment.rs");

    match path {
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {}
            _ => panic!("{}", error),
        },
        Ok(f) => {
            // Initialise as an empty file
            write!(&f, "").unwrap();
        }
    }
}
