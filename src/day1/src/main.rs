use std::path::PathBuf;
use utils::read_lines;

const BASE_10: u32 = 10;

fn main() {
    let path = PathBuf::from("src/day1/day1.txt");
    let input = read_lines(path);

    println!("{}", sum_of_calibration_numbers(input));
}

fn sum_of_calibration_numbers(input: Vec<String>) -> u64 {
    input.iter().map(|line| {
        let first = line.chars().find(|c| c.is_ascii_digit()).and_then(|f| f.to_digit(BASE_10)).unwrap();
        let last = line.chars().rev().find(|c| c.is_ascii_digit()).and_then(|f| f.to_digit(BASE_10)).unwrap();
        (first * 10 + last) as u64
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_test() {
        let input = vec![
            "1abc2", // 12
            "pqr3stu8vwx", // 38
            "a1b2c3d4e5f", // 15
            "treb7uchet", // 77
        ];
        let input = input.iter().map(|s| String::from(*s)).collect();

        assert_eq!(sum_of_calibration_numbers(input), 142);
    }
}