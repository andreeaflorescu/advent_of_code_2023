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
    // Defines if the hand is played with `J` as a joker.
    with_joker: bool,
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

impl Hand {
    fn new(value: &str, with_joker: bool) -> Self {
        let mut cards = HashMap::new();

        let (hand, rank) = value.trim().split_once(' ').unwrap();
        // To compare the cards in the order in which they appear we just convert the
        // hand to a hex number and just compare numbers afterwards.
        // 'T' -> 'A'
        // 'J' => 'B' or '1' if we play with Joker
        // 'Q' => 'C'
        // 'K' => 'D'
        // 'A' => 'E'
        let j_replacement = if with_joker { '1' } else { 'B' };
        let hex = hand
            .replace('A', "E")
            .replace('K', "D")
            .replace('Q', "C")
            .replace('J', &j_replacement.to_string())
            .replace('T', "A");
        for c in hex.chars() {
            cards.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        if with_joker {
            if let Some(num_jokers) = cards.remove(&j_replacement) {
                match cards.values().max() {
                    Some(max) => {
                        let best_card = cards
                            .iter()
                            .filter(|(_, v)| **v == *max)
                            .max_by(|a, b| a.1.cmp(b.1))
                            .unwrap();
                        cards.entry(*best_card.0).and_modify(|v| *v += num_jokers);
                    }
                    None => {
                        // It can be the case that we only had `J` in a hand. In this case
                        // max will return None, and we have no more cards in our hands.
                        // In this scenario we can just insert the highest possible card.
                        cards.insert('A', 5);
                    }
                }
            }
        }
        Hand {
            cards,
            bid: rank.parse::<usize>().unwrap(),
            hex: usize::from_str_radix(hex.as_str(), 16).unwrap(),
            with_joker,
        }
    }

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

impl Hands {
    fn new(input: &[String], with_joker: bool) -> Self {
        let mut inner = input
            .iter()
            .map(|s| Hand::new(s.as_str(), with_joker))
            .collect::<Vec<Hand>>();
        inner.sort_by(|a, b| b.cmp(a));
        Self { inner }
    }

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
    let hands = Hands::new(&input, false);
    println!("Part 1: {}", hands.total_winnings()); // 249748283
    let hands = Hands::new(&input, true);
    println!("Part 2: {}", hands.total_winnings()); // 248029057
}

#[cfg(test)]
mod tests {
    use crate::{Hand, HandType, Hands};

    #[test]
    fn test_values_comparison() {
        let full_a = Hand::new("AAAAA 123", false);
        let full_j = Hand::new("JJJJJ 123", false);
        assert!(full_a > full_j);

        let h1 = Hand::new("22222 1", false);
        let h2 = Hand::new("AAAAK 1", false);
        assert_eq!(h1._type(), HandType::FiveOfAKind);
        assert_eq!(h2._type(), HandType::FourOfAKind);
        assert!(h1 > h2);

        let h1 = Hand::new("22345 1", false);
        assert_eq!(h1._type(), HandType::Pair);
        let h2 = Hand::new("AKQJT 1", false);

        assert_eq!(h2._type(), HandType::HighCard);
        assert!(h1 > h2);

        let h1 = Hand::new("A224A 1", false);
        assert_eq!(h1._type(), HandType::TwoPairs);
        let h2 = Hand::new("KKQQJ 1", false);
        assert_eq!(h2._type(), HandType::TwoPairs);
        assert!(h1 > h2);

        let h1 = Hand::new("22223 1", false);
        let h2 = Hand::new("AAAKK 1", false);
        assert!(h1 > h2);

        let h1 = Hand::new("22333 1", false);
        let h2 = Hand::new("AA222 1", false);
        assert!(h2 > h1);

        let h1 = Hand::new("77888 11", false);
        assert_eq!(h1._type(), HandType::FullHouse);
        let h2 = Hand::new("77788 2", false);
        assert_eq!(h2._type(), HandType::FullHouse);
        assert!(h1 > h2);

        let h1 = Hand::new("33332 1", false);
        let h2 = Hand::new("2AAAA 2", false);
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
        let hands = Hands::new(input, false);
        assert_eq!(hands.total_winnings(), 6440);
    }

    #[test]
    fn test_part2() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
        let hands = Hands::new(input, true);
        assert_eq!(hands.total_winnings(), 5905);
    }
}
