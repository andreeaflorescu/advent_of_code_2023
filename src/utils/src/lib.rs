use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

pub fn read_lines(path: PathBuf) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
}

// Create a vector from numbers separated by one or more ' '. T must be a numeric type.
pub fn as_vec<T: FromStr>(input: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    input.split(' ').filter(|s| !s.is_empty()).map(|v|v.parse::<T>().unwrap()).collect::<Vec<T>>()
}
