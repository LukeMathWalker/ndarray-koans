#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use regex::Regex;
use std::ffi::OsString;
use std::fs::{read_dir, FileType, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct KoanCollection {
    path: String,
    enlightenment_path: String,
    koans: Vec<Koan>,
}

impl KoanCollection {
    pub fn new(path: &str, enlightenment_path: &str) -> Self {
        let mut koans: Vec<(OsString, OsString)> = read_dir(path)
            .unwrap()
            .map(|f| {
                let entry = f.unwrap();
                // Each entry in path has to be a directory!
                assert!(
                    entry.file_type().unwrap().is_dir(),
                    "Each entry in {:} has to be a directory",
                    path
                );
                let directory_name = entry.file_name();
                read_dir(entry.path())
                    .unwrap()
                    .map(move |f| (directory_name.to_owned(), f.unwrap().file_name()))
            })
            .flatten()
            .collect();
        // Sort them in lexicographical order - koans are prefixed with `dd_`
        koans.sort();

        Self {
            path: path.to_string(),
            enlightenment_path: enlightenment_path.to_string(),
            koans: koans.into_iter().map(|f| f.into()).collect(),
        }
    }

    pub fn n_opened(&self) -> usize {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.enlightenment_path)
            .unwrap();
        BufReader::new(&file).lines().count()
    }

    pub fn opened(&self) -> impl Iterator<Item = &Koan> {
        self.koans.iter().take(self.n_opened())
    }

    pub fn next(&self) -> Option<&Koan> {
        let n_opened = self.n_opened();
        if n_opened == self.koans.len() {
            None
        } else {
            Some(&self.koans[n_opened])
        }
    }

    pub fn open_next(&mut self) -> Result<&Koan, ()> {
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .write(true)
            .open(&self.enlightenment_path)
            .unwrap();

        let koan = self.next();
        if let Some(koan) = koan {
            let koan_filename: String = koan.into();
            writeln!(file, "include!(\"koans/{:}.rs\");", koan_filename).unwrap();
            Ok(koan)
        } else {
            Err(())
        }
    }
}

pub struct Koan {
    pub parent_name: String,
    pub parent_number: String,
    pub name: String,
    pub number: usize,
}

impl From<(OsString, OsString)> for Koan {
    fn from(x: (OsString, OsString)) -> Self {
        let (parent_dir_name, filename) = x;
        let filename = filename.into_string().unwrap();
        let parent_dir_name = parent_dir_name.into_string().unwrap();

        let re = Regex::new(r"(?P<number>\d{2})_(?P<name>\w+)\.rs").unwrap();
        let (name, number) = match re.captures(&filename) {
            None => panic!("Failed to parse koan name."),
            Some(s) => {
                let name = s["name"].into();
                let number = s["number"].parse().unwrap();
                (name, number)
            }
        };

        let re = Regex::new(r"(?P<number>\d{2})_(?P<name>\w+)").unwrap();
        let (parent_name, parent_number) = match re.captures(&parent_dir_name) {
            None => panic!("Failed to parse dir name."),
            Some(s) => {
                let name = s["name"].into();
                let number = s["number"].parse().unwrap();
                (name, number)
            }
        };

        Koan {
            parent_name,
            parent_number,
            name,
            number,
        }
    }
}

impl Into<String> for &Koan {
    fn into(self) -> String {
        format!(
            "{:02}_{}/{:02}_{}",
            &self.parent_number, &self.parent_name, &self.number, &self.name
        )
    }
}
