use crate::utils;

use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
struct Card {
    id: usize,
    winners: Vec<usize>,
    mine: Vec<usize>,
    copies: usize,
}

impl Card {
    pub fn score(&self) -> usize {
        let win_set: HashSet<_> = self.winners.iter().collect();
        let my_set: HashSet<_> = self.mine.iter().collect();
        let l = my_set
            .intersection(&win_set)
            .clone()
            .collect::<Vec<_>>()
            .len();
        l
    }
}

fn parse_numbers(nmbrs: &str) -> Vec<usize> {
    nmbrs
        .trim()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
fn parse_card(s: &str) -> Card {
    //Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    let c = s.split(':').collect::<Vec<&str>>();
    let id = c
        .first()
        .unwrap()
        .replace("Card ", "")
        .trim()
        .parse::<usize>()
        .unwrap();

    let winners_mine = c.last().unwrap().split('|').collect::<Vec<&str>>();
    let winners = winners_mine.first().unwrap();
    let mine = winners_mine.last().unwrap();

    let winners = parse_numbers(winners);
    let mine = parse_numbers(mine);

    Card {
        id,
        winners,
        mine,
        copies: 1,
    }
}
fn parse_card_set(lines: Vec<&str>) -> CardSet {
    let cards = lines.iter().map(|l| parse_card(l)).collect::<Vec<_>>();
    CardSet { cards }
}
#[derive(Debug, Clone)]
struct CardSet {
    pub cards: Vec<Card>,
}

impl CardSet {
    fn final_score(&self) -> usize {
        self.cards.iter().fold(0, |acc, nxt| {
            let s = nxt.score();
            match s {
                0 => acc,
                _ => acc + 2usize.pow((s - 1) as u32),
            }
        })
    }
}
pub fn part1() {
    let lines = utils::load_input("../data/day4.txt");
    let cs = parse_card_set(lines);
    println!("Day 4 Part 1: {}", cs.final_score());
}
pub fn part2() {
    let lines = utils::load_input("../data/day4.txt");

    let mut cards = parse_card_set(lines).cards;
    for i in 0..cards.len() - 1 {
        let current = cards.get(i).unwrap();
        let score = current.score();
        let copies = current.copies;
        if score > 0 {
            for ix in i + 1..i + 1 + score {
                let c = cards.get_mut(ix).unwrap();
                c.copies += copies;
            }
        }
    }

    let l = cards.iter().fold(0, |acc, c| acc + c.copies);

    println!("Day 4 Part 2: {}", l);
}

#[cfg(test)]
mod tests {}
