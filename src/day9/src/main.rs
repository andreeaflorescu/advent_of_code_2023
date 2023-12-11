use std::path::PathBuf;
use utils::{as_vec, read_lines};

struct Report {
    inner: Vec<ValueHistory>,
}

#[derive(Debug)]
struct ValueHistory {
    inner: Vec<isize>,
}

impl From<&String> for ValueHistory {
    fn from(value: &String) -> Self {
        let inner = as_vec(value.as_str());
        ValueHistory { inner }
    }
}

fn compute_diffs_array(inner: &[isize]) -> Vec<isize> {
    inner
        .windows(2)
        .map(|v| v[1] - v[0])
        .collect::<Vec<isize>>()
}

fn all_zeroes(inner: &[isize]) -> bool {
    inner.iter().all(|v| *v == 0)
}

impl ValueHistory {
    fn predict_next(&self) -> isize {
        let mut diffs = compute_diffs_array(&self.inner);
        let mut next_value = *diffs.last().unwrap();
        while !all_zeroes(&diffs) {
            diffs = compute_diffs_array(&diffs);
            next_value += *diffs.last().unwrap_or(&0);
        }

        next_value + self.inner.last().unwrap()
    }

    fn predict_previous(&self) -> isize {
        let mut diffs = compute_diffs_array(&self.inner);

        // We keep the difference from the first extrapolation in `previous_diff`.
        // For finding out how much is `previous_diff`, we have to use the
        // following formula:
        // val_0 = val_1 - diff_1[0];
        // diff_1[0] = diff_1[1] - diff_2[1];
        // diff_2[0] = diff_2[1] - diff_3[1];
        // ...
        // Thus, val_0 = val_1 - diff_1[1] + diff_2[1] - diff_3[1] + diff4[1] ....
        // In this notation diff_1[0] we convey the following meaning:
        //  * _n -> the extrapolation number
        //  * diff_n -> array containing all the nth extrapolations
        let mut previous_diff = diffs[0];
        let mut iter = 1;
        while !all_zeroes(&diffs) {
            diffs = compute_diffs_array(&diffs);
            if iter % 2 == 0 {
                previous_diff += diffs[0];
            } else {
                previous_diff -= diffs[0];
            }
            iter += 1;
        }
        self.inner[0] - previous_diff
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

    fn predict_previous_sum(&self) -> isize {
        self.inner.iter().map(|v| v.predict_previous()).sum()
    }
}

fn main() {
    let path = PathBuf::from("src/day9/input.txt");
    let input = read_lines(path);
    let report = Report::from(input);
    println!("Part 1: {}", report.predict_next_sum());
    println!("Part 2: {}", report.predict_previous_sum());
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

        let history = vec![0, 3, 6, 9, 12, 15];
        let history = ValueHistory { inner: history };
        assert_eq!(history.predict_next(), 18);
        let report = Report::from(input);
        assert_eq!(report.predict_next_sum(), 114);
    }

    #[test]
    fn test_predict_prev() {
        let input = "10 13 16 21 30 45".to_string();
        let history = ValueHistory::from(&input);
        assert_eq!(history.predict_previous(), 5);

        let input = "0 3 6 9 12 15".to_string();
        let history = ValueHistory::from(&input);
        assert_eq!(history.predict_previous(), -3);

        let input = "1 3 6 10 15 21".to_string();
        let history = ValueHistory::from(&input);
        assert_eq!(history.predict_previous(), 0);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .lines()
            .map(String::from)
            .collect();

        let report = Report::from(input);
        assert_eq!(report.predict_previous_sum(), 2);
    }
}
