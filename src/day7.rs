use crate::day7::Hand::{Fives, Fours, FullHouse, HighCard, OnePair, Threes, TwoPair};
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::utils;
use itertools::Itertools;
use std::fs::read_to_string;
use std::mem::discriminant;
use std::panic::panic_any;

static LABEL: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
#[derive(Debug, Eq, PartialEq)]
struct HandStr(String);
impl Ord for HandStr {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_hands(&self.0, &other.0)
    }
}

impl PartialOrd for HandStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(self))
    }
}

// impl Eq for HandStr {}
// impl PartialEq for HandStr {
//     fn eq(&self, other: &Self) -> bool {
//         false
//     }
//
//     fn ne(&self, other: &Self) -> bool {
//         false
//     }
// }

//Eq, PartialEq, Ord and PartialOrd.
fn value(ch: char) -> usize {
    let mut ix: usize = 0;
    for c in LABEL.iter() {
        if *c == ch {
            return ix;
        } else {
            ix += 1;
        }
    }
    panic!();
}
fn compare_hands(hand1: &str, hand2: &str) -> Ordering {
    let chars1 = hand1.chars().collect::<Vec<_>>();
    let chars2 = hand2.chars().collect::<Vec<_>>();

    if value(*chars2.first().unwrap()) < value(*chars1.first().unwrap()) {
        return Ordering::Less;
    }
    if value(*chars2.first().unwrap()) > value(*chars1.first().unwrap()) {
        return Ordering::Greater;
    }

    compare_hands(&hand1[1..], &hand2[1..])
}

#[derive(Eq, Ord, Debug, PartialEq, PartialOrd)]
struct Bid(usize);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
enum Hand {
    HighCard(HandStr, usize),
    OnePair(HandStr, usize),
    TwoPair(HandStr, usize),
    Threes(HandStr, usize),
    FullHouse(HandStr, usize),
    Fours(HandStr, usize),
    Fives(HandStr, usize),
}
fn parse_hand(s: &str, bid: usize) -> Hand {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        if map.contains_key(&c) {
            map.insert(c, map.get(&c).unwrap() + 1);
        } else {
            map.insert(c, 1);
        }
    }

    let mut vals = map.values().collect::<Vec<&usize>>();
    vals.sort();
    vals.reverse();

    if vals.len() == 1usize {
        return Fives(HandStr(s.to_string()), bid);
    }
    if vals.len() == 2usize {
        if vals[0] == &4usize {
            return Fours(HandStr(s.to_string()), bid);
        } else {
            // ==3
            return FullHouse(HandStr(s.to_string()), bid);
        }
    }
    if vals.len() == 3usize {
        if vals[0] == &3usize {
            return Threes(HandStr(s.to_string()), bid);
        } else {
            return TwoPair(HandStr(s.to_string()), bid);
        }
    }
    if vals.len() == 4usize {
        return OnePair(HandStr(s.to_string()), bid);
    }
    HighCard(HandStr(s.to_string()), bid)
}

fn parse_input() -> Vec<(String, usize)> {
    let data = utils::load_input("../data/day7.txt");
    let mut res = vec![];
    for d in data {
        let (card, bid) = d.split_once(' ').unwrap();
        res.push((card.to_string(), bid.parse::<usize>().unwrap()));
    }
    res
}

// Five of a kind, where all five cards have the same label: AAAAA
// Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// High card, where all cards' labels are distinct: 23456
fn hand_str(h: Hand) -> String {
    match h {
        HighCard(HandStr(s), _)
        | OnePair(HandStr(s), _)
        | TwoPair(HandStr(s), _)
        | Threes(HandStr(s), _)
        | FullHouse(HandStr(s), _)
        | Fours(HandStr(s), _)
        | Fives(HandStr(s), _) => s,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use crate::utils::load_input;
    use itertools::all;
    use load_file::load_str;
    use std::mem;

    // 32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483

    #[test]
    fn p1() {
        let mut d = parse_input();
        let mut v = d
            .iter()
            .map(|(h, bid)| parse_hand(h, *bid))
            .collect::<Vec<_>>();
        v.sort_by(|a, b| a.cmp(b));
        v.reverse();
        let mut groups = vec![];
        for (_key, group) in &v.iter().group_by(|x1| mem::discriminant(*x1)) {
            let group_vec: Vec<_> = group.collect();
            groups.push(group_vec);
        }

        let mut flatnd = vec![];
        for g in groups {
            let mut x = g;

            x.sort_by(|a, b| a.cmp(b));
            flatnd.extend(x);
        }
        flatnd.reverse();
        let mut ix = 1;
        let mut sum = 0;
        for h in flatnd {
            let bid = match h {
                HighCard(HandStr(s), n)
                | OnePair(HandStr(s), n)
                | TwoPair(HandStr(s), n)
                | Threes(HandStr(s), n)
                | FullHouse(HandStr(s), n)
                | Fours(HandStr(s), n)
                | Fives(HandStr(s), n) => n,
            };
            sum += ix * bid;
            ix += 1;
        }
        println!("{:?}", sum);
    }
    #[test]
    fn test_data() {
        let h1 = parse_hand("32T3K", 765);
        println!("{:?}", h1);

        let h2 = parse_hand("T55J5", 684);
        println!("{:?}", h2);

        let h3 = parse_hand("KK677", 28);
        println!("{:?}", h3);

        let h4 = parse_hand("KTJJT", 220);
        println!("{:?}", h4);

        let h5 = parse_hand("QQQJA", 483);
        println!("{:?}", h5);

        let mut v = vec![h1, h2, h3, h4, h5];

        v.sort_by(|a, b| a.cmp(b));
        v.reverse();
        let mut groups = vec![];
        for (_key, group) in &v.iter().group_by(|x1| mem::discriminant(*x1)) {
            let group_vec: Vec<_> = group.collect();
            groups.push(group_vec);
        }

        let mut flatnd = vec![];
        for g in groups {
            let mut x = g;

            x.sort_by(|a, b| a.cmp(b));
            flatnd.extend(x);
        }
        flatnd.reverse();
        let mut ix = 1;
        let mut sum = 0;
        for h in flatnd {
            let bid = match h {
                HighCard(HandStr(s), n)
                | OnePair(HandStr(s), n)
                | TwoPair(HandStr(s), n)
                | Threes(HandStr(s), n)
                | FullHouse(HandStr(s), n)
                | Fours(HandStr(s), n)
                | Fives(HandStr(s), n) => n,
            };
            sum += ix * bid;
            ix += 1;
        }
        println!("{:?}", sum);
    }
}
