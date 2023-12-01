use std::fs::{read_to_string};
use std::path::PathBuf;

pub fn read_lines(path: PathBuf) -> Vec<String> {
    read_to_string(path).unwrap().lines().map(String::from).collect::<Vec<String>>()
}