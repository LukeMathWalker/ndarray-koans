#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use regex::Regex;
use std::convert::TryInto;
use std::ffi::OsString;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

struct Koan {
    name: String,
    number: usize,
}

impl From<OsString> for Koan {
    fn from(filename: OsString) -> Self {
        let filename = filename.into_string().unwrap();
        let re = Regex::new(r"(?P<number>\d{2})_(?P<name>\w+)\.rs").unwrap();
        match re.captures(&filename) {
            None => panic!("Failed to parse koan name."),
            Some(s) => Koan {
                name: s["name"].into(),
                number: s["number"].parse().unwrap(),
            },
        }
    }
}

impl Into<String> for Koan {
    fn into(self) -> String {
        format!("{:02}_{}", self.number, self.name)
    }
}

fn main() {
    let message = if seek_the_path() {
        "Eternity lies ahead of us, and behind. Your path is not yet finished."
    } else {
        "What is the sound of one hand clapping (for you)?"
    };

    println!("{}", message);
}

macro_rules! koan {
    ($name:expr) => {
        include!(concat!("koans/", $name, ".rs"));
    };
}

fn seek_the_path() -> bool {
    let koans = get_koans();
    let mut path = OpenOptions::new()
        .read(true)
        .append(true)
        .open("src/path_to_enlightenment.rs")
        .unwrap();
    let n_opened_koans = BufReader::new(&path).lines().count();

    print!(" \n\n\n");
    for koan in koans.iter().take(n_opened_koans) {
        if !run_tests(Some(&koan.name), false) {
            println!("\tKoan: {} ❌", koan.name);
            println!("\nMeditate on your approach and return. Mountains are merely mountains.");
            run_tests(Some(&koan.name), true);
            return false;
        } else {
            println!("\tKoan: {} ✔️", koan.name);
        }
    }

    if let Some(next_koan) = koans.into_iter().nth(n_opened_koans) {
        println!("Ahead of you lies {:?}.", next_koan.name);
        let koan_filename: String = next_koan.into();
        write!(&mut path, "koan!(\"{:}\");\n", koan_filename).unwrap();
        true
    } else {
        println!("There will be no more tasks.");
        false
    }
}

fn get_koans() -> Vec<Koan> {
    let mut koans: Vec<OsString> = read_dir("src/koans")
        .unwrap()
        .into_iter()
        .map(|f| f.unwrap().file_name())
        .collect();
    // Sort them in lexicographical order - koans are prefixed with `dd_`
    koans.sort();
    koans.into_iter().map(|f| f.into()).collect()
}

fn run_tests(filter: Option<&str>, output: bool) -> bool {
    let std = || {
        if output {
            Stdio::inherit()
        } else {
            Stdio::null()
        }
    };
    let mut args = vec!["test", "-q"];

    if let Some(test_filter) = filter {
        args.push(test_filter);
    }

    Command::new("cargo")
        .args(args)
        .stdout(std())
        .stderr(std())
        .status()
        .unwrap()
        .success()
}

#[cfg(test)]
mod path_to_enlightenment;
