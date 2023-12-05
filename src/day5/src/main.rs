use std::path::PathBuf;
use utils::read_lines;

struct SeedRange {
    source: usize,
    destination: usize,
    length: usize,
}

impl SeedRange {
    fn new(source: usize, destination: usize, length: usize) -> Self {
        Self {
            source,
            destination,
            length
        }
    }

    // Maps `source` to a destination, returns `None` if `source` is not in range.
    fn map_source(&self, source: usize) -> Option<usize> {
        if source >= self.source && source < self.source + self.length {
            let offset = source - self.source;
            return Some(self.destination + offset)
        }
        return None
    }
}

enum Category {
    Seed = 0x0,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

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
            map: Vec::new()
        }
    }

    // Returns the destination of the passed `source`.
    fn map_source_value(&self, source: usize) -> usize {
        self.map.iter().find_map(|r| r.map_source(source)).unwrap_or(source)
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<SeedMap>,
}

impl From<Vec<String>> for Almanac {
    fn from(value: Vec<String>) -> Self {
        todo!()
    }
}

impl Almanac {
    fn find_lowest_location(&self) -> usize {
        todo!()
    }
}

fn main() {
    let path = PathBuf::from("src/day5/day5.txt");
    let input = read_lines(path);
    let almanac = Almanac::from(input);
    println!("Part 1: {}", almanac.find_lowest_location());
}

#[cfg(test)]
mod tests {
    use crate::{Category, SeedMap};
    use super::SeedRange;

    #[test]
    fn test_range() {
        let r = SeedRange {
            source: 0,
            destination: 10,
            length: 5,
        };
        assert_eq!(r.map_source(0), Some(10));
        assert_eq!(r.map_source(5), None);
        assert_eq!(r.map_source(4), Some(14));
    }

    #[test]
    fn test_map_range() {
        let seed_map = SeedMap {
            source_category: Category::Soil,
            destination_category: Category::Seed,
            map: vec![
                SeedRange::new(0, 5, 10),
                SeedRange::new(20, 30, 5),
            ],
        };

        assert_eq!(seed_map.map_source_value(5), 10);
        assert_eq!(seed_map.map_source_value(21), 31);
        assert_eq!(seed_map.map_source_value(10), 10);
        assert_eq!(seed_map.map_source_value(15), 15);
        assert_eq!(seed_map.map_source_value(39), 39);
    }
}
