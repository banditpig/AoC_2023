use crate::day2::Colour::Red;
use crate::utils::load_input;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Colour {
    Red,
    Green,
    Blue,
}
#[derive(Debug, Default)]
pub struct CubeSet {
    pub(crate) red: usize,
    pub(crate) green: usize,
    pub(crate) blue: usize,
}
impl CubeSet {
    pub fn passes(&self, constraint: &CubeSet) -> bool {
        self.red <= constraint.red && self.green <= constraint.green && self.blue <= constraint.blue
    }
}
#[derive(Debug, Default)]
pub struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}
impl Game {
    fn passes(&self, constraint: &CubeSet) -> bool {
        self.sets.iter().all(|g| g.passes(constraint))
    }
}

#[derive(Debug, Default)]
pub struct GameSet {
    pub(crate) games: Vec<Game>,
}
impl GameSet {
    pub fn passing_games(&self, constraint: &CubeSet) -> Vec<usize> {
        self.games
            .iter()
            .filter(|g| g.passes(constraint))
            .map(|g| g.id)
            .collect::<Vec<usize>>()
    }
    pub fn passing_games_sum(&self, constraint: &CubeSet) -> usize {
        self.passing_games(constraint).iter().sum()
    }
}

fn parse_one_colour(s: &str) -> (Colour, usize) {
    let mut color_map = HashMap::new();
    color_map.insert(Colour::Red, 0);
    color_map.insert(Colour::Green, 0);
    color_map.insert(Colour::Blue, 0);

    let s = s.trim().split(' ').collect::<Vec<&str>>();
    let n = s.first().unwrap().parse::<usize>().unwrap();
    match *s.last().unwrap() {
        "red" => (Colour::Red, n),
        "green" => (Colour::Green, n),
        "blue" => (Colour::Blue, n),

        _ => todo!(),
    }
}
pub fn parse_one_set(s: &str) -> CubeSet {
    //eg: 3 red, 2 blue,  4 green
    //or: 2 red, 1 green
    //split on ',' and trim
    //map parse_one_colour over each to give
    let single_colors = s.split(",").collect::<Vec<&str>>();
    let colour_tuples = single_colors
        .iter()
        .map(|sc| parse_one_colour(sc))
        .collect::<Vec<(Colour, usize)>>();
    let mut color_map = HashMap::new();
    color_map.insert(Colour::Red, 0);
    color_map.insert(Colour::Green, 0);
    color_map.insert(Colour::Blue, 0);

    for (c, size) in colour_tuples {
        color_map.insert(c, size);
    }
    let cs = CubeSet {
        red: *color_map.get(&Colour::Red).unwrap(),
        green: *color_map.get(&Colour::Green).unwrap(),
        blue: *color_map.get(&Colour::Blue).unwrap(),
    };
    cs
}
pub fn parse_one_game(line: &str) -> Game {
    let g = line.split(":").collect::<Vec<&str>>();
    let game_id = g
        .first()
        .unwrap()
        .replace("Game ", "")
        .parse::<usize>()
        .unwrap();
    //
    let rest = g.last().unwrap();
    let sets = rest.split(";").collect::<Vec<&str>>();
    let sets = sets
        .iter()
        .map(|set| parse_one_set(set))
        .collect::<Vec<CubeSet>>();
    Game {
        id: game_id,
        sets: sets,
    }
}
pub fn parse_games() -> Vec<Game> {
    let lines = load_input("../data/day2.txt");
    lines
        .iter()
        .map(|g| parse_one_game(g))
        .collect::<Vec<Game>>()
}
// Game 1: 3 blue, 4 red;  1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    pub fn one_game() {
        let s = "Game 1: 3 blue, 4 red;  1 red, 2 green, 6 blue; 2 green";
        let g = parse_one_game(s);
        assert_eq!(g.id, 1);

        let bl = g.sets.get(0).unwrap().blue;
        assert_eq!(3, bl);
        let gr = g.sets.get(0).unwrap().green;
        assert_eq!(0, gr);
        let rd = g.sets.get(0).unwrap().red;
        assert_eq!(4, rd);
        //-------------------
        let bl = g.sets.get(1).unwrap().blue;
        assert_eq!(6, bl);
        let gr = g.sets.get(1).unwrap().green;
        assert_eq!(2, gr);
        let rd = g.sets.get(1).unwrap().red;
        assert_eq!(1, rd);
        //-------------------
        let bl = g.sets.get(2).unwrap().blue;
        assert_eq!(0, bl);
        let gr = g.sets.get(2).unwrap().green;
        assert_eq!(2, gr);
        let rd = g.sets.get(2).unwrap().red;
        assert_eq!(0, rd);
    }
    #[test]
    pub fn one_set() {
        let s = "8 green, 6 blue, 20 red";
        let c = parse_one_set(s);
        assert_eq!(8, c.green);
        assert_eq!(6, c.blue);
        assert_eq!(20, c.red);

        let s = "8 green ";
        let c = parse_one_set(s);
        assert_eq!(8, c.green);

        let s = "8 green, 20 red ";
        let c = parse_one_set(s);
        assert_eq!(8, c.green);
        assert_eq!(0, c.blue);
        assert_eq!(20, c.red);
    }
}
