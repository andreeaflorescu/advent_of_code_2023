use std::path::PathBuf;
use utils::read_lines;

struct Report {
    inner: Vec<ValueHistory>,
}

struct ValueHistory {
    inner: Vec<isize>,
}

impl From<&String> for ValueHistory {
    fn from(value: &String) -> Self {
        let inner = value
            .split(' ')
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        ValueHistory { inner }
    }
}

impl ValueHistory {
    fn predict_next(&self) -> isize {
        let mut diffs = self
            .inner
            .windows(2)
            .map(|v| v[1] - v[0])
            .collect::<Vec<isize>>();
        let mut next_value = *diffs.last().unwrap();
        while !diffs.iter().all(|v| *v == 0) {
            diffs = diffs
                .windows(2)
                .map(|v| v[1] - v[0])
                .collect::<Vec<isize>>();
            next_value += *diffs.last().unwrap_or(&0);
        }

        next_value + self.inner.last().unwrap()
    }
}

impl From<Vec<String>> for Report {
    fn from(value: Vec<String>) -> Self {
        let inner = value
            .iter()
            .map(ValueHistory::from)
            .collect::<Vec<ValueHistory>>();
        Report { inner }
    }
}

impl Report {
    fn predict_next_sum(&self) -> isize {
        self.inner.iter().map(|v| v.predict_next()).sum()
    }
}

fn main() {
    let path = PathBuf::from("src/day9/input.txt");
    let input = read_lines(path);
    let report = Report::from(input);
    println!("Part 1: {}", report.predict_next_sum());
}

#[cfg(test)]
mod tests {
    use crate::{Report, ValueHistory};

    #[test]
    fn test_part1() {
        let input: Vec<String> = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .lines()
            .map(String::from)
            .collect();
        let first = 18;
        let second = 28;
        let third = 68;

        let history = vec![0, 3, 6, 9, 12, 15];
        let history = ValueHistory { inner: history };
        assert_eq!(history.predict_next(), 18);
        let report = Report::from(input);
        assert_eq!(report.predict_next_sum(), 114);
    }
}
