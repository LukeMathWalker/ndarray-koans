#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use std::process::Command;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ffi::OsString;

fn main() {
    let message = if run_tests() {
        if seek_the_path() {
            "Eternity lies ahead of us, and behind. Your path is not yet finished."
        } else {
            "What is the sound of one hand clapping (for you)?"
        }
    } else {
        "Meditate on your approach and return. Mountains are merely mountains."
    };

    println!("{}", message);
}

macro_rules! koan {
    ($name:expr) => {
        include!(concat!("koans/", $name));
    };
}

fn seek_the_path() -> bool {
    let koans = get_koans();
    let mut path = OpenOptions::new()
        .read(true)
        .append(true)
        .open("src/path_to_enlightenment.rs")
        .unwrap();
    let n_solved_koans = BufReader::new(&path).lines().count();

    if let Some(next_koan) = koans.into_iter().nth(n_solved_koans) {
        println!("Ahead of you lies {:?}.", next_koan);
        write!(&mut path, "koan!({:?});\n", next_koan).unwrap();
        true
    } else {
        println!("There will be no more tasks.");
        false
    }
}

fn get_koans() -> Vec<OsString> {
    read_dir("src/koans")
        .unwrap()
        .into_iter()
        .map(|f| f.unwrap().file_name())
        .collect()
}

fn run_tests() -> bool {
    Command::new("cargo")
        .arg("test")
        .arg("-q")
        .status()
        .unwrap()
        .success()
}

#[cfg(test)]
mod path_to_enlightenment;
