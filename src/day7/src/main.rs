use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use utils::read_lines;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPairs = 2,
    TreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    hex: usize,
    cards: HashMap<char, usize>,
    bid: usize,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type = self._type();
        let other_hand_type = other._type();
        if self._type() != other._type() {
            hand_type.cmp(&other_hand_type)
        } else {
            self.hex.cmp(&other.hex)
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut cards = HashMap::new();
        let (value, rank) = value.trim().split_once(' ').unwrap();
        for c in value.chars() {
            cards.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        // To compare the cards in the order in which they appear we just convert the
        // hand to a hex number and just compare numbers afterwards.
        // 'T' -> 'A'
        // 'J' => 'B'
        // 'Q' => 'C'
        // 'K' => 'D'
        // 'A' => 'E'
        let hex = value
            .replace('A', "E")
            .replace('K', "D")
            .replace('Q', "C")
            .replace('J', "B")
            .replace('T', "A");
        Hand {
            cards,
            bid: rank.parse::<usize>().unwrap(),
            hex: usize::from_str_radix(hex.as_str(), 16).unwrap(),
        }
    }
}

impl Hand {
    fn _type(&self) -> HandType {
        let vals = self.cards.values().copied().collect::<Vec<usize>>();
        match vals.len() {
            1 => HandType::FiveOfAKind, // a single value means 5 of a kind
            2 => {
                if vals.contains(&4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if vals.contains(&3) {
                    HandType::TreeOfAKind
                } else {
                    HandType::TwoPairs
                }
            }
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => panic!("Invalid number of cards"),
        }
    }
}

#[derive(Debug)]
struct Hands {
    inner: Vec<Hand>,
}

impl From<Vec<String>> for Hands {
    fn from(value: Vec<String>) -> Self {
        let mut inner = value
            .iter()
            .map(|s| Hand::from(s.as_str()))
            .collect::<Vec<Hand>>();
        inner.sort_by(|a, b| b.cmp(a));
        Self { inner }
    }
}

impl Hands {
    fn total_winnings(&self) -> usize {
        let len = self.inner.len();
        self.inner
            .iter()
            .enumerate()
            .map(|(rank, hand)| (len - rank) * hand.bid)
            .sum()
    }
}

fn main() {
    let path = PathBuf::from("src/day7/src/input.txt");
    let input = read_lines(path);
    let hands = Hands::from(input);
    println!("Part 1: {}", hands.total_winnings()); // 249748283
}

#[cfg(test)]
mod tests {
    use crate::{Hand, HandType, Hands};

    #[test]
    fn test_values_comparison() {
        let full_a = Hand::from("AAAAA 123");
        let full_j = Hand::from("JJJJJ 123");
        assert!(full_a > full_j);

        let h1 = Hand::from("22222 1");
        let h2 = Hand::from("AAAAK 1");
        assert_eq!(h1._type(), HandType::FiveOfAKind);
        assert_eq!(h2._type(), HandType::FourOfAKind);
        assert!(h1 > h2);

        let h1 = Hand::from("22345 1");
        assert_eq!(h1._type(), HandType::Pair);
        let h2 = Hand::from("AKQJT 1");

        assert_eq!(h2._type(), HandType::HighCard);
        assert!(h1 > h2);

        let h1 = Hand::from("A224A 1");
        assert_eq!(h1._type(), HandType::TwoPairs);
        let h2 = Hand::from("KKQQJ 1");
        assert_eq!(h2._type(), HandType::TwoPairs);
        assert!(h1 > h2);

        let h1 = Hand::from("22223 1");
        let h2 = Hand::from("AAAKK 1");
        assert!(h1 > h2);

        let h1 = Hand::from("22333 1");
        let h2 = Hand::from("AA222 1");
        assert!(h2 > h1);

        let h1 = Hand::from("77888 11");
        assert_eq!(h1._type(), HandType::FullHouse);
        let h2 = Hand::from("77788 2");
        assert_eq!(h2._type(), HandType::FullHouse);
        assert!(h1 > h2);

        let h1 = Hand::from("33332 1");
        let h2 = Hand::from("2AAAA 2");
        assert!(h1 > h2);
    }

    #[test]
    fn test_cmp_hand_type() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::TreeOfAKind);
        assert!(HandType::TreeOfAKind > HandType::TwoPairs);
        assert!(HandType::TwoPairs > HandType::Pair);
        assert!(HandType::Pair > HandType::HighCard);
    }

    #[test]
    fn test_part1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
        let hands = Hands::from(input);
        assert_eq!(hands.total_winnings(), 6440);
    }
}
