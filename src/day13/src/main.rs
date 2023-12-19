use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use utils::read_lines;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Reflection {
    Column(usize),
    Row(usize),
}

impl Display for Reflection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(r) => write!(f, "Row {r}"),
            Self::Column(c) => write!(f, "Column {c}"),
        }
    }
}

#[derive(Debug)]
struct LavaIsland {
    mirrors: Vec<Mirror>,
}

impl LavaIsland {
    fn new(input: Vec<String>) -> Self {
        let mut start = 0;
        let mut end = 0;
        let mut map = Vec::new();
        for (i, line) in input.iter().enumerate() {
            if line.is_empty() {
                map.push(Mirror::new(&input[start..=end]));
                start = i + 1;
                end = i + 1;
            } else {
                end = i;
            }
        }
        map.push(Mirror::new(&input[start..=end]));
        LavaIsland { mirrors: map }
    }

    fn part_1(&self) -> usize {
        let mut sum = 0;
        for mirror in self.mirrors.iter() {
            match mirror.find_reflection() {
                Reflection::Column(c) => {
                    sum += c + 1;
                }
                Reflection::Row(r) => {
                    sum += (r + 1) * 100;
                }
            }
        }
        sum
    }
}

#[derive(Debug)]
struct Mirror {
    inner: Vec<Vec<char>>,
}

impl Display for Mirror {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let disp = self
            .inner
            .iter()
            .map(String::from_iter)
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{disp}")
    }
}

impl Mirror {
    fn rows(&self) -> usize {
        self.inner.len()
    }

    fn columns(&self) -> usize {
        self.inner[0].len()
    }

    fn new(input: &[String]) -> Self {
        Self {
            inner: input
                .iter()
                .map(|line| line.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        }
    }

    fn is_reflection_at_column(&self, col: usize) -> bool {
        let mut left;
        let mut right;
        for row in 0..self.rows() {
            left = col;
            right = col + 1;
            while right < self.columns() {
                if self.inner[row][left] != self.inner[row][right] {
                    return false;
                }
                match left.checked_sub(1) {
                    Some(l) => left = l,
                    None => break,
                };
                right += 1;
            }
        }
        true
    }

    fn is_reflection_at_line(&self, line: usize) -> bool {
        let mut left = line;
        let mut right = line + 1;
        while right < self.rows() {
            if self.inner[left] != self.inner[right] {
                return false;
            }
            match left.checked_sub(1) {
                Some(l) => left = l,
                None => break,
            };
            right += 1;
        }

        true
    }

    fn find_reflection(&self) -> Reflection {
        for row in 0..self.rows() - 1 {
            if self.is_reflection_at_line(row) {
                return Reflection::Row(row);
            }
        }

        for column in 0..self.columns() - 1 {
            if self.is_reflection_at_column(column) {
                return Reflection::Column(column);
            }
        }

        panic!("no reflection found for {}", self);
    }
}

fn main() {
    let input = read_lines(PathBuf::from("src/day13/input.txt"));
    let island = LavaIsland::new(input);
    println!("Part1: {}", island.part_1()); // 37113
}

#[cfg(test)]
mod tests {
    use crate::{LavaIsland, Mirror};

    #[test]
    fn test_line_reflection() {
        let input = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();
        let mirrors = Mirror::new(input.as_slice());
        assert_eq!(mirrors.is_reflection_at_line(3), true);
    }

    #[test]
    fn test_part1() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();
        let map = LavaIsland::new(input);
        assert_eq!(map.part_1(), 405);
    }
}
