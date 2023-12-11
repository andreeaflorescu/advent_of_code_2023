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

pub fn with_boarder(value: Vec<String>, boarder_char: char) -> Vec<Vec<char>> {
    let mut inner = Vec::new();
    let boarder_row = vec![boarder_char; value[0].len() + 2];
    // 1. The first line needs to be a boarder.
    inner.push(boarder_row.clone());
    // 2. We're now pushing the actual input rows.
    for row in value.iter() {
        // Each column must start and end with the boarder char.
        inner.push(
            format!("{boarder_char}{row}{boarder_char}")
                .chars()
                .collect(),
        );
    }
    // 3. The last line needs to be a boarder.
    inner.push(boarder_row);
    inner
}
