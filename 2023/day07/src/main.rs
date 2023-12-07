use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(inp: &str) -> i64 {
    let mut hands = parse_v1(inp);
    hands.sort_by(|a, b| {
        let c = a.kind.cmp(&b.kind);
        if matches!(c, std::cmp::Ordering::Equal) {
            for i in 0..5 {
                let a = a.cards[i];
                let b = b.cards[i];
                if a != b {
                    return a.cmp(&b);
                }
            }
        }
        c
    });

    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank as i64 + 1) * h.bid)
        .sum()
}

fn part2(inp: &str) -> i64 {
    let mut hands = parse_v2(inp);
    hands.sort_by(|a, b| {
        let c = a.kind.cmp(&b.kind);
        if matches!(c, std::cmp::Ordering::Equal) {
            for i in 0..5 {
                let a = a.cards[i];
                let b = b.cards[i];
                if a != b {
                    if a == Card::J {
                        return std::cmp::Ordering::Less;
                    }
                    if b == Card::J {
                        return std::cmp::Ordering::Greater;
                    }
                    return a.cmp(&b);
                }
            }
        }
        c
    });

    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank as i64 + 1) * h.bid)
        .sum()
}

fn parse_v1(inp: &str) -> Vec<Hand> {
    let mut res = Vec::new();
    let mut count = HashMap::<_, i64>::new();
    for line in inp.lines() {
        let Some((hand, bid)) = line.split_once(' ') else {
            continue;
        };
        let hand = hand.as_bytes();
        debug_assert_eq!(hand.len(), 5);
        let cards = [
            hand[0].into(),
            hand[1].into(),
            hand[2].into(),
            hand[3].into(),
            hand[4].into(),
        ];

        count.clear();
        for c in cards {
            *count.entry(c).or_default() += 1;
        }

        let kind;
        match count.len() {
            1 => kind = Kind::FiveOfAKind,
            2 => {
                let max = *count.values().max().unwrap();
                if max == 4 {
                    kind = Kind::FourOfAKind
                } else {
                    kind = Kind::FullHouse
                }
            }
            3 => {
                let max = *count.values().max().unwrap();
                if max == 3 {
                    kind = Kind::ThreeOfAKind
                } else {
                    kind = Kind::TwoPair
                }
            }
            4 => kind = Kind::OnePair,
            5 => kind = Kind::High,
            _ => unreachable!("{count:?}"),
        }

        res.push(Hand {
            cards,
            kind,
            bid: bid.parse().unwrap(),
        })
    }
    res
}

fn parse_v2(inp: &str) -> Vec<Hand> {
    let mut res = Vec::new();
    let mut count = HashMap::<_, i64>::new();
    for line in inp.lines() {
        let Some((hand, bid)) = line.split_once(' ') else {
            continue;
        };
        let hand = hand.as_bytes();
        debug_assert_eq!(hand.len(), 5);
        let cards = [
            hand[0].into(),
            hand[1].into(),
            hand[2].into(),
            hand[3].into(),
            hand[4].into(),
        ];

        count.clear();
        for c in cards {
            *count.entry(c).or_default() += 1;
        }

        let kind;
        if count.contains_key(&Card::J) {
            let c = count[&Card::J];
            if c != 5 {
                let max = count
                    .iter_mut()
                    .filter(|(k, _v)| **k != Card::J)
                    .max_by_key(|(_k, v)| **v)
                    .unwrap();
                *max.1 += c;
                count.remove(&Card::J);
            }
        }
        match count.len() {
            1 => kind = Kind::FiveOfAKind,
            2 => {
                let max = *count.values().max().unwrap();
                if max == 4 {
                    kind = Kind::FourOfAKind
                } else {
                    kind = Kind::FullHouse
                }
            }
            3 => {
                let max = *count.values().max().unwrap();
                if max == 3 {
                    kind = Kind::ThreeOfAKind
                } else {
                    kind = Kind::TwoPair
                }
            }
            4 => kind = Kind::OnePair,
            5 => kind = Kind::High,
            _ => unreachable!("{count:?}"),
        }

        res.push(Hand {
            cards,
            kind,
            bid: bid.parse().unwrap(),
        })
    }
    res
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    kind: Kind,
    bid: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<u8> for Card {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::T,
            b'J' => Card::J,
            b'Q' => Card::Q,
            b'K' => Card::K,
            b'A' => Card::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    High = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_p1() {
        let res = part1(INPUT);
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_p2() {
        let res = part2(INPUT);
        assert_eq!(res, 5905);
    }
}
