use std::path::PathBuf;
use regex::Regex;
use utils::read_lines;

fn broken_spring(len: usize) -> String {
    format!("[#?]{{{len}}}{}", working_spring())
}

const fn working_spring() -> &'static str {
    "(?:[^#]+?|\\A|\\z)"
}

fn regex_from(config: &str) -> Regex {
    let mut regex = working_spring().to_string();
    let broken_config = config.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    broken_config.iter().for_each(|num| {
        regex += broken_spring(*num).as_str();
    });
    regex = format!("^{regex}$");

    Regex::new(&regex).unwrap()
}

struct SpringRecords {
    config: Vec<String>,
}

impl SpringRecords {
    fn new(springs: Vec<String>) -> Self {
        Self {
            config: springs
        }
    }

    fn combinations_sum(&self, repeat: bool) -> usize {
        self.config.iter().enumerate().map(|(i, config)| {
            println!("Iteration: {i}");
            let (haystack, broken_config) = config.split_once(' ').unwrap();
            let mut haystack = haystack.to_string();
            let mut broken_config = broken_config.to_string();
            if repeat {
                let repeated = vec![haystack; 5];
                haystack = repeated.join("?");

                let repeated = vec![broken_config; 5];
                broken_config = repeated.join(",");
            }
            let regex = regex_from(broken_config.as_str());
            count_matches(haystack, &regex)
        }).sum()
    }
}

fn count_matches(haystack: String, regex: &Regex) -> usize {
    if regex.is_match(haystack.as_str()) {
        if !haystack.contains('?') {
            return 1;
        } else {
            return
                count_matches(haystack.replacen('?', ".", 1), regex)
                    + count_matches(haystack.replacen('?', "#", 1), regex);
        }
    } else {
        // we reached a dead end
        return 0;
    }
}

fn main() {
    let path = PathBuf::from("src/day12/input.txt");
    let input = read_lines(path);
    let springs = SpringRecords::new(input);
    println!("Part 1: {}", springs.combinations_sum(false)); // 7163
    println!("Part 2: {}", springs.combinations_sum(true));
}

#[cfg(test)]
mod tests {
    use crate::{count_matches, regex_from, SpringRecords};

    #[test]
    fn test_combinations_sum() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#.lines().map(String::from).collect::<Vec<String>>();
        let springs = SpringRecords::new(input);
        assert_eq!(springs.combinations_sum(false), 21);
        assert_eq!(springs.combinations_sum(true), 525152);
    }

    #[test]
    fn test_substitution() {
        let regex = regex_from("1,1,3");
        let solutions = count_matches("???.###".to_string(), &regex);
        assert_eq!(solutions, 1);

        let solutions = count_matches(".??..??...?##.".to_string(), &regex);
        assert_eq!(solutions, 4);

        let regex = regex_from("1,3,1,6");
        let solutions = count_matches("?#?#?#?#?#?#?#?".to_string(), &regex);
        assert_eq!(solutions, 1);

        let regex = regex_from("1,6,5");
        let solutions = count_matches("????.######..#####.".to_string(), &regex);
        assert_eq!(solutions, 4);

        let regex = regex_from("4,1,1");
        let solutions = count_matches("????.#...#...".to_string(), &regex);
        assert_eq!(solutions, 1);

        let regex = regex_from("3,2,1");
        let solutions = count_matches("?###????????".to_string(), &regex);
        assert_eq!(solutions, 10);
    }
}
