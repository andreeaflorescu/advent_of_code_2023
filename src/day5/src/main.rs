use std::path::PathBuf;
use utils::read_lines;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct SeedRange {
    source: usize,
    destination: usize,
    length: usize,
}

impl SeedRange {
    #[cfg(test)]
    fn new(source: usize, destination: usize, length: usize) -> Self {
        Self {
            source,
            destination,
            length,
        }
    }

    // Maps `source` to a destination, returns `None` if `source` is not in range.
    fn find_destination_for(&self, source: usize) -> Option<usize> {
        if source >= self.source && source < self.source + self.length {
            let offset = source - self.source;
            return Some(self.destination + offset);
        }
        None
    }
}

impl From<&str> for SeedRange {
    fn from(value: &str) -> Self {
        let tokens = value
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Self {
            source: tokens[1],
            destination: tokens[0],
            length: tokens[2],
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
enum Category {
    #[default]
    Seed = 0x0,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Category {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct SeedMap {
    source_category: Category,
    destination_category: Category,
    map: Vec<SeedRange>,
}

impl SeedMap {
    fn new(source_category: Category, destination_category: Category) -> SeedMap {
        SeedMap {
            source_category,
            destination_category,
            map: Vec::new(),
        }
    }

    fn push_range(&mut self, range: SeedRange) {
        self.map.push(range);
    }

    // Returns the destination of the passed `source`.
    fn find_destination_for(&self, source: usize) -> usize {
        self.map
            .iter()
            .find_map(|r| r.find_destination_for(source))
            .unwrap_or(source)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<SeedMap>,
}

impl From<Vec<String>> for Almanac {
    fn from(lines: Vec<String>) -> Self {
        const SEEDS_START_LINE: &str = "seeds: ";
        let seeds = lines[0][SEEDS_START_LINE.len()..]
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut maps = Vec::new();
        let mut seed_map = SeedMap::default();
        // the second line is empty, we just skip it.
        for line in lines[2..].iter() {
            if line.contains("map") {
                // we're starting a new SeedMap.
                let (mapping, _) = line.split_once(' ').unwrap();
                let (source, destination) = mapping.split_once("-to-").unwrap();
                seed_map = SeedMap::new(
                    Category::try_from(source).unwrap(),
                    Category::try_from(destination).unwrap(),
                );
            } else if line.is_empty() {
                // The configuration of one seed map is done. We need to push it to `maps`.
                maps.push(seed_map.clone());
            } else {
                seed_map.push_range(SeedRange::from(line.as_str()));
            }
        }
        // We need to push the last created seedsmap.
        maps.push(seed_map);

        Self { seeds, maps }
    }
}

impl Almanac {
    fn find_location_for_seed(&self, seed: &usize) -> usize {
        self.maps
            .iter()
            .fold(*seed, |res, seed_map| seed_map.find_destination_for(res))
    }

    fn find_lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.find_location_for_seed(seed))
            .min()
            .unwrap()
    }

    fn find_lowest_location_with_seed_range(&self) -> usize {
        // this has to go through 1680883088 numbers, so it's rather slow.
        self.seeds
            .chunks(2)
            .map(|window| {
                let start = window[0];
                let end = start + window[1];
                (start..end)
                    .map(|seed| self.find_location_for_seed(&seed))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

fn main() {
    let path = PathBuf::from("src/day5/day5.txt");
    let input = read_lines(path);
    let almanac = Almanac::from(input);
    println!("Part 1: {}", almanac.find_lowest_location());
    println!("Part 2: {}", almanac.find_lowest_location_with_seed_range()); // 99751240
}

#[cfg(test)]
mod tests {
    use super::SeedRange;
    use crate::{Almanac, Category, SeedMap};

    fn test_input() -> Vec<String> {
        r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
            .to_string()
            .lines()
            .map(String::from)
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_range() {
        let r = SeedRange {
            source: 0,
            destination: 10,
            length: 5,
        };
        assert_eq!(r.find_destination_for(0), Some(10));
        assert_eq!(r.find_destination_for(5), None);
        assert_eq!(r.find_destination_for(4), Some(14));
    }

    #[test]
    fn test_map_range() {
        let seed_map = SeedMap {
            source_category: Category::Soil,
            destination_category: Category::Seed,
            map: vec![SeedRange::new(0, 5, 10), SeedRange::new(20, 30, 5)],
        };

        assert_eq!(seed_map.find_destination_for(5), 10);
        assert_eq!(seed_map.find_destination_for(21), 31);
        assert_eq!(seed_map.find_destination_for(10), 10);
        assert_eq!(seed_map.find_destination_for(15), 15);
        assert_eq!(seed_map.find_destination_for(39), 39);
    }

    #[test]
    fn test_parse_almanac() {
        let input = test_input();
        let almanac = Almanac::from(input);

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            almanac.maps.last().unwrap(),
            &SeedMap {
                source_category: Category::Humidity,
                destination_category: Category::Location,
                map: vec![SeedRange::new(56, 60, 37), SeedRange::new(93, 56, 4),],
            }
        );
        assert_eq!(almanac.maps[0].find_destination_for(79), 81);
    }

    #[test]
    fn test_part1() {
        let input = test_input();
        let almanac = Almanac::from(input);

        assert_eq!(almanac.find_lowest_location(), 35);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let almanac = Almanac::from(input);

        assert_eq!(almanac.find_lowest_location_with_seed_range(), 46);
    }
}
