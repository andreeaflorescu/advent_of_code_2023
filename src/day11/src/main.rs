use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use utils::read_lines;

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq)]
struct Position {
    line: usize,
    column: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

impl Position {
    fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    fn distance_to(&self, other: &Position) -> usize {
        self.column.abs_diff(other.column) + self.line.abs_diff(other.line)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Image {
    galaxies: Vec<Position>,
}

impl Image {
    fn galaxy_pairs(&self) -> Vec<[Position; 2]> {
        let mut pairs = Vec::new();
        for i in 0..self.galaxies.len() {
            for j in 0..self.galaxies.len() {
                if i != j {
                    let mut pair = [self.galaxies[i], self.galaxies[j]];
                    pair.sort();
                    if !pairs.contains(&pair) {
                        pairs.push(pair);
                    }
                }
            }
        }
        pairs
    }
    fn sum_of_shortest_path(&self) -> usize {
        let pairs = self.galaxy_pairs();
        pairs.iter().map(|pair| pair[0].distance_to(&pair[1])).sum()
    }
}

fn parse_galaxies(input: Vec<String>) -> Vec<Position> {
    let mut galaxies = Vec::new();

    for (i, line) in input.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Position::new(i, j));
            }
        }
    }

    galaxies
}

impl From<Vec<String>> for Image {
    fn from(lines: Vec<String>) -> Self {
        let mut galaxies = parse_galaxies(lines);

        let mut empty_columns = Vec::new();
        let max_column = galaxies
            .iter()
            .max_by(|this, that| this.column.cmp(&that.column))
            .unwrap()
            .column;
        let min_column = galaxies
            .iter()
            .min_by(|this, that| this.column.cmp(&that.column))
            .unwrap()
            .column;
        for column in min_column..max_column {
            if !galaxies.iter().any(|g| g.column == column) {
                empty_columns.push(column);
            }
        }

        let mut empty_lines = Vec::new();
        let max_line = galaxies
            .iter()
            .max_by(|this, that| this.line.cmp(&that.line))
            .unwrap()
            .line;
        let min_line = galaxies
            .iter()
            .min_by(|this, that| this.line.cmp(&that.line))
            .unwrap()
            .line;
        for line in min_line..max_line {
            if !galaxies.iter().any(|g| g.line == line) {
                empty_lines.push(line);
            }
        }

        for g in galaxies.iter_mut() {
            let count = empty_columns.iter().filter(|p| **p < g.column).count();
            g.column += count;
            let count = empty_lines.iter().filter(|p| **p < g.line).count();
            g.line += count;
        }

        Image { galaxies }
    }
}

fn main() {
    let path = PathBuf::from("src/day11/input.txt");
    let lines = read_lines(path);
    let image = Image::from(lines);
    println!("Part 1: {}", image.sum_of_shortest_path()); // 9918828
}

#[cfg(test)]
mod tests {
    use crate::{parse_galaxies, Image, Position};

    fn part_1_input() -> Vec<String> {
        r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
            .lines()
            .map(String::from)
            .collect()
    }

    fn part_1_expansion() -> Vec<String> {
        r#"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."#
            .lines()
            .map(String::from)
            .collect()
    }

    #[test]
    fn test_expansion() {
        let input = part_1_input();
        let expansion = part_1_expansion();
        let mut image = Image::from(input);

        let mut expected_position = parse_galaxies(expansion);
        expected_position.sort();
        image.galaxies.sort();
        assert_eq!(image.galaxies, expected_position);
    }

    #[test]
    fn test_distance_between_points() {
        let p1 = Position::new(6, 1);
        let p2 = Position::new(12, 5);
        assert_eq!(p1.distance_to(&p2), 10);
        assert_eq!(p2.distance_to(&p1), 10);

        let p1 = Position::new(11, 0);
        let p2 = Position::new(11, 5);
        assert_eq!(p1.distance_to(&p2), 5);
    }

    #[test]
    fn test_part1() {
        let input: Vec<String> = part_1_input();
        let image = Image::from(input);
        assert_eq!(image.sum_of_shortest_path(), 374);
    }
}
