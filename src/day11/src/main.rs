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
            for j in i+1..self.galaxies.len() {
                if i != j {
                    let pair = [self.galaxies[i], self.galaxies[j]];
                    pairs.push(pair);
                }
            }
        }
        pairs
    }

    fn sum_of_shortest_path(&self) -> usize {
        let pairs = self.galaxy_pairs();
        pairs.iter().map(|pair| pair[0].distance_to(&pair[1])).sum()
    }

    fn expand_columns(&mut self) {
        let mut columns = self
            .galaxies
            .iter()
            .map(|p| p.column)
            .collect::<Vec<usize>>();
        columns.sort();

        let max_column = *columns.last().unwrap();
        let mut offsets = Vec::new();
        let mut empty_counter = 0;
        for c in 0..=max_column {
            if !columns.contains(&c) {
                empty_counter += 1;
            }
            offsets.push(empty_counter);
        }

        self.galaxies
            .iter_mut()
            .for_each(|g| g.column += offsets[g.column]);
    }
}

fn parse_galaxies(input: Vec<String>) -> Vec<Position> {
    let mut galaxies = Vec::new();

    let mut x = 0;
    let mut any_galaxy;
    for line in input.iter() {
        any_galaxy = false;
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Position::new(x, j));
                any_galaxy = true;
            }
        }
        // if we don't have any galaxies on the line, we need to expand the space.
        if !any_galaxy {
            x += 1;
        }
        x += 1;
    }

    galaxies
}

impl From<Vec<String>> for Image {
    fn from(lines: Vec<String>) -> Self {
        let mut image = Image {
            galaxies: parse_galaxies(lines),
        };

        image.expand_columns();

        image
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
    use crate::{Image, Position};

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
