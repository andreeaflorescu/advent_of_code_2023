use std::path::PathBuf;
use utils::{as_vec, read_lines};

fn number_from_str(input: &str) -> usize {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[derive(Debug, Clone, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Self { time, distance }
    }

    fn count_possible_wins(&self) -> usize {
        let first_win = self.find_lowest_button_hold().unwrap();
        let last_win = self.find_highest_button_hold(first_win).unwrap();
        last_win - first_win + 1
    }

    fn is_winning_distance(&self, dist: usize) -> bool {
        self.distance < dist
    }

    fn calculate_distance_for_time_pressed(&self, time_pressed: usize) -> usize {
        self.time.saturating_sub(time_pressed) * time_pressed
    }

    // find highest time to hold the button such that it yields a distance
    // greater than the record. we don't need to start at 0, we just start
    // at the minimum button press that yields a win.
    fn find_highest_button_hold(&self, start: usize) -> Option<usize> {
        let mut s = start;
        let mut end = self.time;
        let mut mid;
        loop {
            mid = (s + end) / 2;
            if s > end {
                return None;
            }
            let distance = self.calculate_distance_for_time_pressed(mid);
            if self.is_winning_distance(distance) {
                if !self.is_winning_distance(self.calculate_distance_for_time_pressed(mid + 1)) {
                    return Some(mid);
                } else {
                    s = mid + 1;
                }
            } else {
                end = mid - 1;
            }
        }
    }

    fn find_lowest_button_hold(&self) -> Option<usize> {
        let mut s = 0;
        let mut end = self.time;
        let mut mid;

        loop {
            mid = (s + end) / 2;
            if s > end {
                return None;
            }

            let distance = self.calculate_distance_for_time_pressed(mid);
            if self.is_winning_distance(distance) {
                if !self.is_winning_distance(self.calculate_distance_for_time_pressed(mid - 1)) {
                    return Some(mid);
                } else {
                    end = mid - 1;
                }
            } else {
                s = mid + 1;
            }
        }
    }
}

impl From<Vec<String>> for Race {
    fn from(value: Vec<String>) -> Self {
        let time = number_from_str(value[0].split_once(':').unwrap().1);
        let distance = number_from_str(value[1].split_once(':').unwrap().1);
        Self { time, distance }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Races {
    inner: Vec<Race>,
}

impl From<Vec<String>> for Races {
    fn from(input: Vec<String>) -> Self {
        let times: Vec<usize> = as_vec(input[0].split_once(':').unwrap().1);
        let distances: Vec<usize> = as_vec(input[1].split_once(':').unwrap().1);
        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(time, dist)| Race::new(*time, *dist))
            .collect::<Vec<Race>>();
        Self { inner: races }
    }
}

impl Races {
    fn multiply_wins(&self) -> usize {
        self.inner
            .iter()
            .map(|race| race.count_possible_wins())
            .product::<usize>()
    }
}

fn main() {
    let path = PathBuf::from("src/day6/src/input.txt");
    let input = read_lines(path);
    let races = Races::from(input.clone());
    println!("Part 1: {}", races.multiply_wins());

    let race = Race::from(input);
    println!("Part 2: {}", race.count_possible_wins());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        let races = Races::from(input);
        assert_eq!(
            races.inner,
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }

    #[test]
    fn test_find_interval() {
        let race = Race::new(7, 9);
        assert_eq!(race.find_lowest_button_hold().unwrap(), 2);
        assert_eq!(race.find_highest_button_hold(2).unwrap(), 5);
    }

    #[test]
    fn test_part1() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        let races = Races::from(input);
        assert_eq!(races.multiply_wins(), 288);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];
        let races = Race::from(input);
        assert_eq!(races.count_possible_wins(), 71503);
    }
}
