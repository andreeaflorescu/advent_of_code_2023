use std::path::PathBuf;
use utils::read_lines;

#[derive(Debug, Default)]
struct Number {
    start_col: usize,
    end_col: usize,
    row: usize,
}

struct EngineSchematic {
    inner: Vec<Vec<char>>,
}

impl Into<EngineSchematic> for Vec<String> {
    fn into(self) -> EngineSchematic {
        EngineSchematic {
            inner: self.iter().map(|row| row.chars().collect()).collect::<Vec<Vec<char>>>()
        }
    }
}

impl EngineSchematic {
    fn num_cols(&self) -> usize {
        self.inner[0].len()
    }

    fn num_rows(&self) -> usize {
        self.inner.len()
    }

    fn generate_neighbor_indexes(&self, num: &Number) -> Vec<(usize, usize)> {
        let mut indexes = Vec::new();

        let mut rows = Vec::new();
        if num.row != 0 {
            rows.push(num.row - 1);
        }
        if num.row != self.num_rows() - 1 {
            rows.push(num.row + 1);
        }

        let col_start = num.start_col.checked_sub(1).unwrap_or(num.start_col);
        let end_col = usize::min(num.end_col + 1, self.num_cols() - 1);
        let cols: Vec<usize> = (col_start..=end_col).collect();

        for r in rows.iter() {
            for c in cols.iter() {
                indexes.push((*r, *c));
            }
        }

        if let Some(c) = num.start_col.checked_sub(1) {
            indexes.push((num.row, c));
        }

        if num.end_col + 1 < self.num_cols() {
            indexes.push((num.row, num.end_col + 1));
        }

        indexes
    }

    fn is_part_number(&self, num: &Number) -> bool {
        let indexes = self.generate_neighbor_indexes(num);

        for i in indexes.iter() {
            let value = self.inner[i.0][i.1];
            if !value.is_ascii_digit() && !(value == '.') {
                return true
            }
        }

        false
    }

    fn as_usize(&self, num: &Number) -> usize {
        self.inner[num.row][num.start_col..=num.end_col].iter().collect::<String>().parse::<usize>().unwrap()
    }

    fn add_part_numbers(&self) -> usize {
        // let mut nums = Vec::new();
        let mut sum = 0;
        for (i, line) in self.inner.iter().enumerate() {
            let mut number: Option<Number> = None;
            for (j, val) in line.iter().enumerate() {
                if val.is_ascii_digit() {
                    if let Some(num) = number.as_mut() {
                        num.end_col = j;
                    } else {
                        number = Some(Number {
                            row: i,
                            start_col: j,
                            end_col: j,
                        });
                    }
                } else {
                    if let Some(num) = number {
                        if self.is_part_number(&num) {
                            sum += self.as_usize(&num);
                        }
                        number = None;
                    }
                }
            }
            if let Some(num) = number {
                if self.is_part_number(&num) {
                    sum += self.as_usize(&num);
                }
                number = None;
            }
        }

    sum
    }
}

fn main() {
    let path = PathBuf::from("src/day3/day3.txt");
    let input = read_lines(path);
    let engine: EngineSchematic = input.into();

    println!("Part 1: {}", engine.add_part_numbers()) // wrong: 523948
}

#[cfg(test)]
mod tests {
    use crate::{EngineSchematic, Number};

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
.664.598.."#.to_string();
        let input = input.lines().map(String::from).collect::<Vec<String>>();
        let engine: EngineSchematic = input.into();

        let num = Number {
            start_col: 2,
            end_col: 3,
            row: 1
        };
        println!("{:#?}", engine.generate_neighbor_indexes(&num));
        assert_eq!(engine.add_part_numbers(), 4361);
    }

    #[test]
    fn test_neighbors() {
        let input = r#"467..114.
...*.....
..35..633
......#..
617*.....
.....+.5.
..592....
......75.
...$.*...
.664.5987"#.to_string();
        let input = input.lines().map(String::from).collect::<Vec<String>>();
        let engine: EngineSchematic = input.into();

        let num = Number {
            start_col: 0,
            end_col: 2,
            row: 0,
        };
        assert_eq!(engine.as_usize(&num), 467);
        let expected_neighbors = vec![(1, 0), (1, 1), (1,2), (1,3), (0, 3)];
        assert_eq!(engine.generate_neighbor_indexes(&num), expected_neighbors);
        assert!(engine.is_part_number(&num));

        let num = Number {
            start_col: 2,
            end_col: 3,
            row: 2
        };
        assert_eq!(engine.as_usize(&num), 35);
        let expected_neighbors = vec![(1, 1), (1, 2), (1, 3), (1, 4), (3, 1), (3, 2), (3, 3), (3, 4), (2, 1), (2, 4)];
        assert_eq!(engine.generate_neighbor_indexes(&num), expected_neighbors);
        assert!(engine.is_part_number(&num));

        let num = Number {
            start_col: 7,
            end_col: 7,
            row: 5,
        };
        assert_eq!(engine.as_usize(&num), 5);
        let expected_neighbors = vec![(4, 6), (4, 7), (4, 8), (6, 6), (6, 7), (6, 8), (5, 6), (5, 8)];
        assert_eq!(engine.generate_neighbor_indexes(&num), expected_neighbors);
        assert!(!engine.is_part_number(&num));

        let num = Number {
            start_col: 5,
            end_col: 8,
            row: 9,
        };
        assert_eq!(engine.as_usize(&num), 5987);
        let expected_neighbors = vec![(8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (9, 4)];
        assert_eq!(engine.generate_neighbor_indexes(&num), expected_neighbors);
        assert!(engine.is_part_number(&num));

        let num = Number {
            start_col: 1,
            end_col: 3,
            row: 9,
        };
        assert_eq!(engine.as_usize(&num), 664);
        let expected_neighbors = vec![(8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (9, 0), (9, 4)];
        assert_eq!(engine.generate_neighbor_indexes(&num), expected_neighbors);
        assert!(engine.is_part_number(&num));
    }

    #[test]
    fn test_part1_from_big_input() {
        let input = r#".........699....*.........=............15*619.......................*......515....487........................808...............*.....611*121
.....369.*.....................813..21.................630...................#.................$....................153........11..........."#;
        let input = input.lines().map(String::from).collect::<Vec<String>>();
        let engine: EngineSchematic = input.into();
        assert_eq!(engine.add_part_numbers(), 699 + 15 + 619 + 515 + 611 + 121 + 11);
    }
}
