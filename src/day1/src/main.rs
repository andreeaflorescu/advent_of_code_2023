use std::path::PathBuf;
use utils::read_lines;

const BASE_10: u32 = 10;

fn main() {
    let path = PathBuf::from("src/day1/day1.txt");
    let input = read_lines(path);

    println!("Part 1: {}", sum_of_calibration_numbers(input.clone()));
    println!("Part 2: {}", sum_of_calibration_numbers_part2(input)); // 52840
}

fn sum_of_calibration_numbers(input: Vec<String>) -> u64 {
    input
        .iter()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(BASE_10)).unwrap();
            let last = line
                .chars()
                .rev()
                .find_map(|c| c.to_digit(BASE_10))
                .unwrap();
            (first * 10 + last) as u64
        })
        .sum()
}

fn sum_of_calibration_numbers_part2(input: Vec<String>) -> u64 {
    let input_with_digits = input
        .iter()
        .map(|line| {
            // Always keep the first and last letter in the digit
            // so that you can account for consecutive digits
            // written with letters that have letters in common.
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .collect();
    sum_of_calibration_numbers(input_with_digits)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_test_part1() {
        let input = vec![
            "1abc2",       // 12
            "pqr3stu8vwx", // 38
            "a1b2c3d4e5f", // 15
            "treb7uchet",  // 77
        ];
        let input = input.iter().map(|s| String::from(*s)).collect();

        assert_eq!(sum_of_calibration_numbers(input), 142);
    }

    #[test]
    fn basic_test_part2() {
        let input = [
            "two1nine",         // 2 9
            "eightwothree",     // 8 3
            "abcone2threexyz",  // 1 3
            "xtwone3four",      // 2 4
            "4nineeightseven2", // 4 2
            "zoneight234",      // 1 4
            "7pqrstsixteen",    // 7 6
        ];
        let input = input.iter().map(|s| String::from(*s)).collect();

        assert_eq!(sum_of_calibration_numbers_part2(input), 281);
    }

    #[test]
    fn test_part2_overlapping() {
        let input = ["seven91sfnbjsccqdtzgleighteightwovqr"];
        let input = input.iter().map(|s| String::from(*s)).collect();
        assert_eq!(sum_of_calibration_numbers_part2(input), 72);
    }
}
