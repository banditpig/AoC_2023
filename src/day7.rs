use crate::utils;
use crate::utils::group_chars;
use itertools::Itertools;
use nom::Or;
use regex::CaptureNames;
use std::cmp::Ordering;
use std::mem;
use std::panic::panic_any;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Threes,
    FullHouse,
    Fours,
    Fives,
}
#[derive(Debug, Eq)]
struct Hand {
    hand_type: HandType,
    bid: usize,
    //score: usize,
    hand_chars: String,
}
fn compare_hand_strings(hand1: &str, hand2: &str) -> Ordering {
    let chars1 = hand1.chars().collect::<Vec<_>>();
    let chars2 = hand2.chars().collect::<Vec<_>>();

    if ch_to_int(*chars1.first().unwrap()) < ch_to_int(*chars2.first().unwrap()) {
        return Ordering::Less;
    }
    if ch_to_int(*chars1.first().unwrap()) > ch_to_int(*chars2.first().unwrap()) {
        return Ordering::Greater;
    }
    compare_hand_strings(&hand1[1..].to_string(), &hand2[1..].to_string())
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type.cmp(&other.hand_type) == Ordering::Equal {
            return true;
        }
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            return compare_hand_strings(&self.hand_chars, &other.hand_chars);
        }
        self.hand_type.cmp(&other.hand_type)
    }
}
fn joker_count(hand: &[char]) -> usize {
    hand.iter().fold(0, |mut acc, nxt| {
        if *nxt == 'J' {
            acc += 1
        }
        acc
    })
}
fn is_joker(ch: char) -> bool {
    ch == 'J'
}
fn ch_to_int(c: char) -> usize {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}
fn determine_hand_type(chars: &[char]) -> HandType {
    //0..4
    use HandType::*;
    if chars[0] == chars[4] {
        return Fives;
    }
    if chars[0] == chars[3] {
        return Fours;
    }
    if chars[0] == chars[2] && chars[3] == chars[4] {
        return FullHouse;
    }
    if chars[0] == chars[2] {
        return Threes;
    }
    if chars[0] == chars[1] && chars[2] == chars[3] {
        return TwoPair;
    }
    if chars[0] == chars[1] {
        return OnePair;
    }
    HighCard
}
fn determine_hand_type_jokers(chars: &[char]) -> HandType {
    //0..4
    use HandType::*;
    let jc = joker_count(chars);
    if jc == 0 {
        return determine_hand_type(chars);
    }

    if chars[0] == chars[4] {
        return Fives;
    }
    if chars[0] == chars[3] {
        return Fours;
    }
    if chars[0] == chars[2] && chars[3] == chars[4] {
        return FullHouse;
    }
    if chars[0] == chars[2] {
        return Threes;
    }
    if chars[0] == chars[1] && chars[2] == chars[3] {
        return TwoPair;
    }
    if chars[0] == chars[1] {
        return OnePair;
    }
    HighCard
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

fn create_hands(input: Vec<(String, usize)>) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for (cards, bid) in input.iter() {
        let mut chars: Vec<char> = cards.chars().collect();
        let grouped_cards: String = group_chars(cards);
        let hand_type = determine_hand_type(grouped_cards.chars().collect::<Vec<_>>().as_slice());

        let h = Hand {
            hand_type,
            bid: *bid,
            hand_chars: cards.clone(),
        };
        hands.push(h);
    }
    hands
}

mod tests {
    use super::*;
    use crate::utils::group_chars;
    use itertools::Itertools;

    #[test]
    fn ranks() {
        let input = parse_input();
        let mut hands = create_hands(input);

        hands.sort();

        let mut sum = 0;
        for (ix, h) in hands.iter().enumerate() {
            sum += (ix + 1) * h.bid;
        }
        //250058342
        //252298357
        dbg!(sum);
    }
}
