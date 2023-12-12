use std::path::PathBuf;
use utils::{read_lines, with_boarder};

struct Map {
    inner: Vec<Vec<char>>,
}

impl From<Vec<String>> for Map {
    fn from(value: Vec<String>) -> Self {
        // We are adding a boarder to the matrix so that we don't need to have special
        // cases for row 0 and N, and column 0 and N.
        const BOARDER_CHAR: char = '.';
        Map {
            inner: with_boarder(value, BOARDER_CHAR),
        }
    }
}

impl Map {
    fn pipe_allowed(&self, position: Position, allowed: Vec<char>) -> Option<Position> {
        if allowed.contains(&self.get(&position)) {
            return Some(position);
        }
        None
    }

    fn pipe_north(&self, position: Position) -> Option<Position> {
        let allowed_north = vec!['|', 'F', '7'];
        self.pipe_allowed(position.north(), allowed_north)
    }

    fn pipe_south(&self, position: Position) -> Option<Position> {
        let allowed_south = vec!['L', 'J', '|'];
        self.pipe_allowed(position.south(), allowed_south)
    }

    fn pipe_east(&self, position: Position) -> Option<Position> {
        let allowed_east = vec!['-', '7', 'J'];
        self.pipe_allowed(position.east(), allowed_east)
    }

    fn pipe_west(&self, position: Position) -> Option<Position> {
        let allowed_west = vec!['-', 'F', 'L'];
        self.pipe_allowed(position.west(), allowed_west)
    }

    fn get(&self, position: &Position) -> char {
        self.inner[position.line][position.column]
    }

    // Find all valid pipe neighbors for `position`. This takes into consideration
    // the orientation of the pipes so it can always return between 0 and maximum 2 neighbors.
    // The notable exception is the `S` for which it might return 4 neighbors if they're all
    // pipes.
    // Returns an empty vector in case there is no pipe as a neighbor.
    fn neighbors(&self, position: Position) -> Vec<Position> {
        let mut neighbors = vec![];
        match self.get(&position) {
            '|' => {
                if let Some(p) = self.pipe_north(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_south(position) {
                    neighbors.push(p);
                }
            }
            '-' => {
                if let Some(p) = self.pipe_west(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_east(position) {
                    neighbors.push(p);
                }
            }
            'L' => {
                if let Some(p) = self.pipe_north(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_east(position) {
                    neighbors.push(p);
                }
            }
            'J' => {
                if let Some(p) = self.pipe_north(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_west(position) {
                    neighbors.push(p);
                }
            }
            '7' => {
                if let Some(p) = self.pipe_south(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_west(position) {
                    neighbors.push(p);
                }
            }
            'F' => {
                if let Some(p) = self.pipe_south(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_east(position) {
                    neighbors.push(p);
                }
            }
            'S' => {
                if let Some(p) = self.pipe_south(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_east(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_north(position) {
                    neighbors.push(p);
                }
                if let Some(p) = self.pipe_west(position) {
                    neighbors.push(p);
                }
            }

            _ => {}
        }
        neighbors
    }

    fn find_farthest_point(&self, start: Position) -> usize {
        let mut visited: Vec<Position> = Vec::new();
        if self.find_loop(start, Position::default(), &mut visited) {
            // the farthest point is just the loop size divided by 2.
            return visited.len() / 2;
        }
        0
    }

    // Returns false in case a loop is not found. The `visited` contains the positions that
    // are part of the loop.
    fn find_loop(&self, current: Position, parent: Position, visited: &mut Vec<Position>) -> bool {
        visited.push(current);
        let neighbors = self.neighbors(current);
        // if there are no neighbors it means that we reached a dead end.
        if neighbors.is_empty() {
            return false;
        }
        for pos in neighbors {
            if !visited.contains(&pos) {
                if self.find_loop(pos, current, visited) {
                    return true;
                }
            } else if pos != parent {
                // if we already visited the current node and the node is not our parent
                // it means that we found the loop.
                return true;
            }
        }
        false
    }

    fn find(&self, value: char) -> Option<Position> {
        for (line, columns) in self.inner.iter().enumerate() {
            for (column, e) in columns.iter().enumerate() {
                if *e == value {
                    return Some(Position::new(line, column));
                }
            }
        }
        None
    }

    fn part_1(&self) -> usize {
        let s_pos = self.find('S').unwrap();
        self.find_farthest_point(s_pos)
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, Ord, PartialOrd, Eq)]
struct Position {
    line: usize,
    column: usize,
}

impl Position {
    fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    fn north(&self) -> Position {
        Position::new(self.line - 1, self.column)
    }

    fn south(&self) -> Position {
        Position::new(self.line + 1, self.column)
    }

    fn west(&self) -> Position {
        Position::new(self.line, self.column - 1)
    }

    fn east(&self) -> Position {
        Position::new(self.line, self.column + 1)
    }
}

fn main() {
    let path = PathBuf::from("src/day10/input.txt");
    let input = read_lines(path);
    let map = Map::from(input);
    println!("Part 1: {}", map.part_1());
}

#[cfg(test)]
mod tests {
    use crate::{Map, Position};

    #[test]
    fn test_part_1() {
        let input: Vec<String> = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ.."#
            .lines()
            .map(String::from)
            .collect();
        let map = Map::from(input);
        let s_position = Position::new(3, 1);
        assert_eq!(map.find('S').unwrap(), s_position);
        let mut neighbors = map.neighbors(s_position);
        neighbors.sort();
        let mut expected_neighbors = vec![s_position.east(), s_position.south()];
        expected_neighbors.sort();
        assert_eq!(neighbors, expected_neighbors);
        assert_eq!(map.part_1(), 8);
    }
}
