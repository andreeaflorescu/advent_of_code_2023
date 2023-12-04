use path::PathBuf;
use std::collections::HashMap;
use std::path;

use utils::read_lines;

#[derive(Debug, PartialEq)]
struct Game {
    sets: Vec<HashMap<String, usize>>,
    id: usize,
}

fn parse_as_games(input: Vec<String>) -> Vec<Game> {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let mut games = Vec::new();

    for line in input {
        let (id, config) = line.split_once(": ").unwrap();
        let id = id.replace("Game ", "").parse::<usize>().unwrap();

        let sets = config.split("; ");
        let mut sets_vec = Vec::new();
        for set in sets {
            let mut h = HashMap::new();
            for cube in set.split(", ") {
                let (number, color) = cube.split_once(' ').unwrap();
                let number = number.parse::<usize>().unwrap();
                h.insert(color.to_string(), number);
            }
            sets_vec.push(h);
        }
        games.push(Game { id, sets: sets_vec })
    }
    games
}

fn is_valid_game_for(game: &Game, criteria: &HashMap<String, usize>) -> bool {
    let colors = criteria.keys().collect::<Vec<&String>>();
    game.sets.iter().all(|s| {
        s.iter().all(|(color, number)| {
            colors.contains(&color) && number <= criteria.get(color).unwrap()
        })
    })
}

fn sum_of_valid_games(games: &[Game], criteria: HashMap<String, usize>) -> usize {
    games
        .iter()
        .filter(|g| is_valid_game_for(g, &criteria))
        .map(|g| g.id)
        .sum()
}

fn min_cubes_for_valid_game(games: &Vec<Game>) -> usize {
    let mut power = 0;

    for game in games {
        let mut hash_set = HashMap::new();
        for (color, number) in game.sets.iter().flatten() {
            hash_set
                .entry(color)
                .and_modify(|v: &mut usize| *v = usize::max(*number, *v))
                .or_insert(*number);
        }
        power += hash_set.values().product::<usize>();
    }

    power
}

pub fn main() {
    let path = PathBuf::from("src/day2/day2.txt");
    let input = read_lines(path);

    let games = parse_as_games(input);
    let criteria = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let sum = sum_of_valid_games(&games, criteria);
    println!("Part 1: {sum}"); // 2331

    let power = min_cubes_for_valid_game(&games);
    println!("Part 2: {power}"); // 71585
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    #[test]
    fn test_parse() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        ];

        let expected_games = vec![
            Game {
                id: 1,
                sets: vec![
                    hashmap!("blue".to_string() => 3, "red".to_string() => 4),
                    hashmap!("red".to_string() => 1, "green".to_string() => 2, "blue".to_string() => 6),
                    hashmap!("green".to_string() => 2),
                ],
            },
            Game {
                id: 2,
                sets: vec![
                    hashmap!("blue".to_string() => 1, "green".to_string() => 2),
                    hashmap!("green".to_string() => 3, "blue".to_string() => 4, "red".to_string() => 1),
                    hashmap!("green".to_string() => 1, "blue".to_string() => 1),
                ],
            },
        ];
        assert_eq!(parse_as_games(input), expected_games);
    }

    #[test]
    fn test_part1() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let input = input.iter().map(|s| String::from(*s)).collect();
        let parsed_input = parse_as_games(input);

        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let criteria = HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]);
        assert_eq!(sum_of_valid_games(&parsed_input, criteria), 8);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let input = input.iter().map(|s| String::from(*s)).collect();
        let parsed_input = parse_as_games(input);

        assert_eq!(min_cubes_for_valid_game(&parsed_input), 2286);
    }
}
