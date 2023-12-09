use std::collections::HashMap;
use std::path::PathBuf;
use utils::read_lines;

fn highest_common_factor(a: usize, b: usize) -> usize {
    let max = usize::min(a, b);
    for i in (1..=max).rev() {
        if a % i == 0 && b % i == 0 {
            return i;
        }
    }
    1
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / highest_common_factor(a, b)
}

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
    fn travel_from(&self, start: &String, destination_pattern: &str) -> usize {
        let mut hops = 0;
        let mut cur = start;
        let mut path_index = 0;

        while !cur.ends_with(destination_pattern) {
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

    fn travel_all(&self) -> usize {
        // We calculate how many hops each of the new input needs to reach a `Z`.
        // Then we just find out what is the least common multiple of all of them.
        let hops = self
            .inner
            .keys()
            .filter(|n| n.ends_with('A'))
            .map(|cur| self.travel_from(cur, "Z"))
            .collect::<Vec<usize>>();
        hops.iter()
            .fold(1, |acc, hops| least_common_multiple(acc, *hops))
    }
}

fn main() {
    let path = PathBuf::from("src/day8/input.txt");
    let input = read_lines(path);
    let map = Map::from(input);
    println!("Part 1: {}", map.travel_from(&"AAA".to_string(), "ZZZ"));
    println!("Part 2: {}", map.travel_all());
}

#[cfg(test)]
mod tests {
    use crate::{least_common_multiple, Map};

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
        assert_eq!(map.travel_from(&"AAA".to_string(), "ZZZ"), 2);

        let input: Vec<String> = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .lines()
            .map(String::from)
            .collect();
        let map = Map::from(input);
        assert_eq!(map.travel_from(&"AAA".to_string(), "ZZZ"), 6);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
            .lines()
            .map(String::from)
            .collect();
        let map = Map::from(input);
        assert_eq!(map.travel_all(), 6);
    }

    #[test]
    fn test_number_ops() {
        assert_eq!(least_common_multiple(5, 15), 15);
        assert_eq!(least_common_multiple(1, 2), 2);
    }
}
