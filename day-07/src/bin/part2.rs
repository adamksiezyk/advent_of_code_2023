use std::{cmp::Ordering, collections::HashMap, iter::zip};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: Type,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        (self.hand_type == other.hand_type) && (self.cards == other.cards)
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => zip(self.cards.chars(), other.cards.chars())
                .map(|(c1, c2)| cmp_card(c1, c2))
                .find(|o| o.is_ne())
                .unwrap_or(Ordering::Equal),
        }
    }
}

fn cmp_card(a: char, b: char) -> Ordering {
    // A, K, Q, J, T
    let val_a = card_to_value(a);
    let val_b = card_to_value(b);
    val_a.cmp(&val_b).reverse()
}

fn card_to_value(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        c => c.to_digit(10).expect("Could not parse card value"),
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let output = part1(input);
    dbg!(output);
}

fn part1(input: String) -> u32 {
    let mut hands = input.lines().map(parse_line).collect::<Vec<_>>();
    hands.sort();
    hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, h)| ((i + 1) as u32) * h.bid)
        .sum()
}

fn parse_line(line: &str) -> Hand {
    let mut parts = line.split_whitespace();
    let cards = parts.next().expect("Could not parse hand");
    let bid = parts
        .next()
        .expect("Could not parse bid")
        .parse::<u32>()
        .expect("Could not parse bid");
    Hand {
        cards: cards.to_string(),
        hand_type: parse_type(cards),
        bid,
    }
}

fn parse_type(cards: &str) -> Type {
    let mut count = HashMap::<char, u32>::new();
    let mut jokers = 0u32;
    for c in cards.chars() {
        match c {
            'J' => jokers += 1,
            _ => {
                let val = match count.get(&c) {
                    Some(v) => v + 1,
                    None => 1,
                };
                count.insert(c, val);
            }
        }
    }
    let mut count = count.values().collect::<Vec<&u32>>();
    count.sort();
    count.reverse();
    let first = **count.get(0).unwrap_or(&&0) + jokers;
    let second = count.get(1);

    match (first, second) {
        (5, _) => Type::FiveOfKind,
        (4, _) => Type::FourOfKind,
        (3, Some(2)) => Type::FullHouse,
        (3, _) => Type::ThreeOfKind,
        (2, Some(2)) => Type::TwoPair,
        (2, _) => Type::OnePair,
        (_, _) => Type::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "T55J5 684";
        let expected = Hand {
            cards: "T55J5".to_string(),
            hand_type: Type::FourOfKind,
            bid: 684,
        };
        let result = parse_line(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sort() {
        let mut input = vec![
            Hand {
                cards: "32T3K".to_string(),
                hand_type: Type::OnePair,
                bid: 765,
            },
            Hand {
                cards: "T55J5".to_string(),
                hand_type: Type::FourOfKind,
                bid: 684,
            },
            Hand {
                cards: "KK677".to_string(),
                hand_type: Type::TwoPair,
                bid: 28,
            },
            Hand {
                cards: "KTJJT".to_string(),
                hand_type: Type::FourOfKind,
                bid: 220,
            },
            Hand {
                cards: "QQQJA".to_string(),
                hand_type: Type::FourOfKind,
                bid: 483,
            },
        ];
        let expected = vec![
            Hand {
                cards: "KTJJT".to_string(),
                hand_type: Type::FourOfKind,
                bid: 220,
            },
            Hand {
                cards: "QQQJA".to_string(),
                hand_type: Type::FourOfKind,
                bid: 483,
            },
            Hand {
                cards: "T55J5".to_string(),
                hand_type: Type::FourOfKind,
                bid: 684,
            },
            Hand {
                cards: "KK677".to_string(),
                hand_type: Type::TwoPair,
                bid: 28,
            },
            Hand {
                cards: "32T3K".to_string(),
                hand_type: Type::OnePair,
                bid: 765,
            },
        ];
        input.sort();
        assert_eq!(input, expected);
    }

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
        let expected = 5905;
        let result = part1(input);
        assert_eq!(result, expected);
    }
}
