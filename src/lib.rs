#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

use regex::Regex;
use std::ffi::OsString;
use std::fs::{read_dir, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct KoanCollection {
    path: String,
    enlightenment_path: String,
    koans: Vec<Koan>,
}

impl KoanCollection {
    pub fn new(path: &str, enlightenment_path: &str) -> Self {
        let mut koans: Vec<OsString> = read_dir(path)
            .unwrap()
            .into_iter()
            .map(|f| f.unwrap().file_name())
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
        let n_opened_koans = BufReader::new(&file).lines().count();
        n_opened_koans
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
            write!(file, "include!(\"koans/{:}.rs\");\n", koan_filename).unwrap();
            Ok(koan)
        } else {
            Err(())
        }
    }
}

pub struct Koan {
    pub name: String,
    pub number: usize,
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

impl Into<String> for &Koan {
    fn into(self) -> String {
        format!("{:02}_{}", &self.number, &self.name)
    }
}
