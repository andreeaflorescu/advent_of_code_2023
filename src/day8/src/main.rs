use std::collections::HashMap;
use std::path::PathBuf;
use utils::read_lines;

struct Map {
    // a hashmap in which the key is the name of the node and the value represents the neighbors
    // in order Left, Right.
    inner: HashMap<String, Vec<String>>,
    path: Vec<usize>,
}

impl From<Vec<String>> for Map {
    fn from(value: Vec<String>) -> Self {
        let path = value[0]
            .replace('R', "1")
            .replace('L', "0")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let map = value[2..]
            .iter()
            .map(|line| {
                let (node, neighbors) = line.split_once(" = ").unwrap();
                let neighbors = neighbors.replace(['(', ')'], "");
                let (l, r) = neighbors.split_once(", ").unwrap();
                (node.to_string(), vec![l.to_string(), r.to_string()])
            })
            .collect();
        Map { path, inner: map }
    }
}

impl Map {
    // Returns how many hops there are between start and destionation.
    fn travel_from(&self, start: String, destination: String) -> usize {
        let mut hops = 0;
        let mut cur = &start;
        let mut path_index = 0;

        while *cur != destination {
            let direction = self.path[path_index];
            cur = &self.inner.get(cur).unwrap()[direction];
            hops += 1;
            path_index += 1;
            if path_index == self.path.len() {
                path_index = 0;
            }
        }
        hops
    }
}

fn main() {
    let path = PathBuf::from("src/day8/input.txt");
    let input = read_lines(path);
    let map = Map::from(input);
    println!(
        "Part 1: {}",
        map.travel_from("AAA".to_string(), "ZZZ".to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_part1() {
        let input: Vec<String> = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
            .lines()
            .map(String::from)
            .collect();
        let map = Map::from(input);
        assert_eq!(map.travel_from("AAA".to_string(), "ZZZ".to_string()), 2);
    }
}
