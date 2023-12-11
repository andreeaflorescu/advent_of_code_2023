use std::collections::HashMap;
use std::path::PathBuf;
use utils::{read_lines, with_boarder};

#[derive(Copy, Clone, Debug, Default)]
struct Number {
    start_col: usize,
    end_col: usize,
    row: usize,
}

impl Number {
    // Create a number that starts and ends at `col`.
    fn new(row: usize, col: usize) -> Number {
        Number {
            row,
            start_col: col,
            end_col: col,
        }
    }

    fn set_end_col(&mut self, end_col: usize) {
        self.end_col = end_col;
    }
}

struct EngineSchematic {
    inner: Vec<Vec<char>>,
}

impl From<Vec<String>> for EngineSchematic {
    fn from(value: Vec<String>) -> EngineSchematic {
        // We are adding a boarder to the matrix so that we don't need to have special
        // cases for row 0 and N, and column 0 and N.
        const BOARDER_CHAR: char = '.';
        EngineSchematic { inner: with_boarder(value, BOARDER_CHAR) }
    }
}

impl EngineSchematic {
    fn generate_neighbor_indexes(&self, num: &Number) -> Vec<(usize, usize)> {
        let mut indexes = Vec::new();

        // The neighbors are the row immediately on top and below the row of the number.
        let rows = [num.row - 1, num.row + 1];
        // The columns we are interested include the diagonal.
        let cols = (num.start_col - 1..=num.end_col + 1).collect::<Vec<usize>>();

        for r in rows.iter() {
            for c in cols.iter() {
                indexes.push((*r, *c));
            }
        }

        // The characters before and after our number on the same line are also neighbors.
        indexes.push((num.row, num.start_col - 1));
        indexes.push((num.row, num.end_col + 1));

        indexes
    }

    fn is_part_number(&self, num: &Number) -> bool {
        let indexes = self.generate_neighbor_indexes(num);

        for i in indexes.iter() {
            let value = self.inner[i.0][i.1];
            // a part number is a number that has at least one neighbor
            // a special character. Special means anything but digits and `.`.
            if !value.is_ascii_digit() && value != '.' {
                return true;
            }
        }

        false
    }

    fn gears(&self) -> HashMap<(usize, usize), Vec<Number>> {
        let numbers = self.numbers();
        let mut gears: HashMap<(usize, usize), Vec<Number>> = HashMap::new();

        for number in numbers {
            let indexes = self.generate_neighbor_indexes(&number);
            for i in indexes.iter() {
                let value = self.inner[i.0][i.1];
                // a gear is a number that has one neighbor `*`.
                if value == '*' {
                    gears
                        .entry((i.0, i.1))
                        .and_modify(|e| e.push(number))
                        .or_insert(vec![number]);
                }
            }
        }
        gears
    }

    fn add_gears(&self) -> usize {
        self.gears()
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| self.as_usize(&v[0]) * self.as_usize(&v[1]))
            .sum()
    }

    fn as_usize(&self, num: &Number) -> usize {
        self.inner[num.row][num.start_col..=num.end_col]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }

    fn numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        for (i, line) in self.inner.iter().enumerate() {
            let mut number: Option<Number> = None;
            for (j, val) in line.iter().enumerate() {
                if val.is_ascii_digit() {
                    match number.as_mut() {
                        Some(num) => num.set_end_col(j),
                        None => number = Some(Number::new(i, j)),
                    };
                } else {
                    if let Some(number) = number {
                        numbers.push(number);
                    }
                    number = None;
                }
            }
        }

        numbers
    }

    fn add_part_numbers(&self) -> usize {
        self.numbers()
            .iter()
            .filter(|n| self.is_part_number(n))
            .map(|n| self.as_usize(n))
            .sum()
    }
}

fn main() {
    let path = PathBuf::from("src/day3/day3.txt");
    let input = read_lines(path);
    let engine: EngineSchematic = input.into();

    println!("Part 1: {}", engine.add_part_numbers()); // 525119
    println!("Part 2: {}", engine.add_gears()); // 76504829
}

#[cfg(test)]
mod tests {
    use crate::EngineSchematic;

    #[test]
    fn part1_test() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .to_string();
        let input = input.lines().map(String::from).collect::<Vec<String>>();
        let engine: EngineSchematic = input.into();
        assert_eq!(engine.add_part_numbers(), 4361);
    }

    #[test]
    fn test_part1_from_big_input() {
        let input = r#".........699....*.........=............15*619.......................*......515....487........................808...............*.....611*121
.....369.*.....................813..21.................630...................#.................$....................153........11..........."#;
        let input = input.lines().map(String::from).collect::<Vec<String>>();
        let engine: EngineSchematic = input.into();
        assert_eq!(
            engine.add_part_numbers(),
            699 + 15 + 619 + 515 + 611 + 121 + 11
        );
    }

    #[test]
    fn test_part2_basic() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
            .to_string();
        let input = input.lines().map(String::from).collect::<Vec<String>>();
        let engine: EngineSchematic = input.into();
        assert_eq!(engine.add_gears(), 467835);
    }
}
