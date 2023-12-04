use std::path::PathBuf;
use utils::read_lines;

struct Card {
    _number: usize,
    winning: Vec<u64>,
    hand: Vec<u64>,
}

impl Card {
    fn score(&self) -> u64 {
        let count = self
            .hand
            .iter()
            .filter(|h| self.winning.contains(h))
            .count();
        match count.checked_sub(1) {
            Some(pow) => 2u64.pow(pow as u32),
            None => 0,
        }
    }
}

fn str_to_vec(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

impl From<&String> for Card {
    fn from(value: &String) -> Self {
        let (card_num, tokens) = value.split_once(':').unwrap();
        let number = card_num
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();

        let (winning, hand) = tokens.split_once('|').unwrap();
        let winning = str_to_vec(winning);
        let hand = str_to_vec(hand);

        Self {
            _number: number,
            winning,
            hand,
        }
    }
}

struct Cards {
    inner: Vec<Card>,
}

impl Cards {
    fn score(&self) -> u64 {
        self.inner.iter().map(|c| c.score()).sum()
    }
}

impl From<Vec<String>> for Cards {
    fn from(value: Vec<String>) -> Self {
        Self {
            inner: value.iter().map(Card::from).collect(),
        }
    }
}

fn main() {
    let path = PathBuf::from("src/day4/day4.txt");
    let input = read_lines(path);
    let cards = Cards::from(input);
    println!("Part 1: {}", cards.score());
}

#[cfg(test)]
mod tests {
    use crate::{Card, Cards};

    #[test]
    fn test_parse_line() {
        let input = "Card   7: 89 70 36 38 86 50 94 62 56  3 |  7  8 56 14 58 65 63 36 54 59 78 79 11  2 69 55 61 39 19 60  4 99 90 17 95".to_string();
        let card = Card::from(&input);

        assert_eq!(card._number, 7);
        assert_eq!(card.winning, vec![89, 70, 36, 38, 86, 50, 94, 62, 56, 3]);
        assert_eq!(
            card.hand,
            vec![
                7, 8, 56, 14, 58, 65, 63, 36, 54, 59, 78, 79, 11, 2, 69, 55, 61, 39, 19, 60, 4, 99,
                90, 17, 95
            ]
        );
    }

    #[test]
    fn test_part1_basic() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .to_string();
        let cards = Cards::from(input.lines().map(String::from).collect::<Vec<String>>());
        assert_eq!(cards.score(), 13);
    }
}
