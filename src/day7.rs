use crate::day7::Hand::{Fives, Fours, FullHouse, HighCard, OnePair, Threes, TwoPair};
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::utils;
use itertools::Itertools;
use std::fs::read_to_string;
use std::mem;
use std::mem::discriminant;
use std::panic::panic_any;

static LABEL: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

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
fn determine_hand(s: &str, bid: usize, vals: &Vec<&usize>) -> Hand {
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
fn parse_raw_hand(s: &str, bid: usize) -> Hand {
    //how many occurences of the cards are there.
    let map = get_card_occurences(s);

    //get them and work from least to most.
    let mut vals = map.values().collect::<Vec<&usize>>();
    vals.sort();
    vals.reverse();
    determine_hand(s, bid, &vals)
}

fn get_card_occurences(s: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        if map.contains_key(&c) {
            map.insert(c, map.get(&c).unwrap() + 1);
        } else {
            map.insert(c, 1);
        }
    }
    map
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

fn sum_bid_rank_products(flatnd: Vec<&Hand>) -> usize {
    //calculate the sum of the rank * bid
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
    sum
}

fn flatten_groups(groups: Vec<Vec<&Hand>>) -> Vec<&Hand> {
    let mut flatnd = vec![];
    for g in groups {
        let mut x = g;
        x.sort_by(|a, b| a.cmp(b));
        flatnd.extend(x);
    }
    flatnd.reverse();
    flatnd
}

fn group_and_sort_hands(v: &Vec<Hand>) -> Vec<Vec<&Hand>> {
    //now group any hands that occur more than once
    //and then sort them on the strengths of the individual cards.
    let mut groups = vec![];
    for (_key, group) in &v.iter().group_by(|x1| mem::discriminant(*x1)) {
        let group_vec: Vec<_> = group.collect();
        groups.push(group_vec);
    }
    groups
}

fn get_hands() -> Vec<Hand> {
    //get hands and sort them in enum Hand's ordering
    let mut d = parse_input();
    let mut v = d
        .iter()
        .map(|(h, bid)| parse_raw_hand(h, *bid))
        .collect::<Vec<_>>();
    v.sort_by(|a, b| b.cmp(a));
    v
}
pub fn part1() {
    let v = get_hands();
    let groups = group_and_sort_hands(&v);
    //flatten the ordered groups
    let flatnd = flatten_groups(groups);
    let sum = sum_bid_rank_products(flatnd);
    println!("{:?}", sum);
}
