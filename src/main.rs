#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use ansi_term::Colour::{Green, Red, White, Yellow};
use ansi_term::Style;
use regex::Regex;
use std::convert::TryInto;
use std::ffi::OsString;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, ExitStatus, Stdio};

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
    let message = if !seek_the_path() {
        "Eternity lies ahead of us, and behind. Your path is not yet finished. 🍂"
    } else {
        if walk_the_path() {
            "Eternity lies ahead of us, and behind. Your path is not yet finished. 🍂"
        } else {
            "What is the sound of one hand clapping (for you)? 🌟"
        }
    };

    println!("\t{}\n", Style::default().italic().paint(message));
}

macro_rules! koan {
    ($name:expr) => {
        include!(concat!("koans/", $name, ".rs"));
    };
}

fn seek_the_path() -> bool {
    let koans = get_koans();
    let path = OpenOptions::new()
        .read(true)
        .append(true)
        .open("src/path_to_enlightenment.rs")
        .unwrap();
    let n_opened_koans = BufReader::new(&path).lines().count();

    print!(" \n\n");
    for koan in koans.iter().take(n_opened_koans) {
        let koan_outcome = run_tests(Some(&koan.name));
        match koan_outcome {
            TestOutcome::Success => {
                println!("\t🚀 {}️", Green.normal().paint(&koan.name));
            }
            TestOutcome::Failure { details } => {
                println!(
                    "\t❌ {}\n\n\t{}\n\n{}",
                    Red.normal().paint(&koan.name),
                    Style::default().italic().paint(
                        "Meditate on your approach and return. Mountains are merely mountains."
                    ),
                    Style::default().dimmed().paint(details)
                );
                return false;
            }
        }
    }
    true
}

fn walk_the_path() -> bool {
    let koans = get_koans();
    let mut path = OpenOptions::new()
        .read(true)
        .append(true)
        .open("src/path_to_enlightenment.rs")
        .unwrap();
    let n_opened_koans = BufReader::new(&path).lines().count();

    if let Some(next_koan) = koans.into_iter().nth(n_opened_koans) {
        println!(
            "{} {}.",
            Yellow.normal().paint("\n\tAhead of you lies"),
            Yellow.bold().paint(&next_koan.name)
        );
        let koan_filename: String = next_koan.into();
        write!(&mut path, "koan!(\"{:}\");\n", koan_filename).unwrap();
        true
    } else {
        println!(
            "{}",
            Green.normal().paint("\n\tThere will be no more tasks.")
        );
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

fn run_tests(filter: Option<&str>) -> TestOutcome {
    let mut args = vec!["test", "-q"];

    if let Some(test_filter) = filter {
        args.push(test_filter);
    }

    let output = Command::new("cargo")
        .args(args)
        .output()
        .expect("Failed to run tests");

    let status = output.status;

    if status.success() {
        TestOutcome::Success
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        TestOutcome::Failure { details: stdout }
    }
}

enum TestOutcome {
    Success,
    Failure { details: String },
}

#[cfg(test)]
mod path_to_enlightenment;
