use crate::utils::load_input;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Colour {
    Red,
    Green,
    Blue,
}
#[derive(Debug, Default, Copy, Clone)]
struct CubeSet {
    pub(crate) red: usize,
    pub(crate) green: usize,
    pub(crate) blue: usize,
}
impl CubeSet {
    pub fn passes(&self, constraint: &CubeSet) -> bool {
        self.red <= constraint.red && self.green <= constraint.green && self.blue <= constraint.blue
    }
    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}
#[derive(Debug, Default)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}
impl Game {
    fn passes(&self, constraint: &CubeSet) -> bool {
        self.sets.iter().all(|g| g.passes(constraint))
    }
    fn min_cube(&self) -> CubeSet {
        let min_cube = CubeSet::default();

        self.sets.iter().fold(min_cube, |mut acc, x| {
            if x.red > acc.red {
                acc.red = x.red;
            }
            if x.green > acc.green {
                acc.green = x.green;
            }
            if x.blue > acc.blue {
                acc.blue = x.blue;
            }
            acc
        })
    }
}

#[derive(Debug, Default)]
struct GameSet {
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
    let s = s.trim().split(' ').collect::<Vec<&str>>();
    let n = s.first().unwrap().parse::<usize>().unwrap();
    match *s.last().unwrap() {
        "red" => (Colour::Red, n),
        "green" => (Colour::Green, n),
        "blue" => (Colour::Blue, n),
        _ => panic!("Won't happen!"),
    }
}
fn parse_one_set(s: &str) -> CubeSet {
    //eg: 3 red, 2 blue,  4 green
    //or: 2 red, 1 green
    //split on ',' and trim

    let single_colors = s.split(',').collect::<Vec<&str>>();
    let folded_map = single_colors
        .iter()
        .map(|sc| parse_one_colour(sc))
        .collect::<Vec<(Colour, usize)>>()
        .iter()
        .fold(HashMap::new(), |mut acc, (c, size)| {
            acc.insert(*c, *size);
            acc
        });

    CubeSet {
        red: *folded_map.get(&Colour::Red).unwrap_or(&0usize),
        green: *folded_map.get(&Colour::Green).unwrap_or(&0usize),
        blue: *folded_map.get(&Colour::Blue).unwrap_or(&0usize),
    }
}

fn parse_one_game(line: &str) -> Game {
    let g = line.split(':').collect::<Vec<&str>>();
    let game_id = g
        .first()
        .unwrap()
        .replace("Game ", "")
        .parse::<usize>()
        .unwrap();
    //
    let rest = g.last().unwrap();
    let sets = rest.split(';').collect::<Vec<&str>>();
    let sets = sets
        .iter()
        .map(|set| parse_one_set(set))
        .collect::<Vec<CubeSet>>();
    Game { id: game_id, sets }
}
fn parse_games() -> Vec<Game> {
    let lines = load_input("../data/day2.txt");
    lines
        .iter()
        .map(|g| parse_one_game(g))
        .collect::<Vec<Game>>()
}
pub fn part1() {
    let gs = parse_games();
    let game_set = GameSet { games: gs };
    let constraint = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum = game_set.passing_games_sum(&constraint);
    println!("Day 2 part 1: {}", sum);
}
pub fn part2() {
    let gs = parse_games();
    let game_set = GameSet { games: gs };
    let s = game_set
        .games
        .iter()
        .map(|g| g.min_cube().power())
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>();

    println!("Day 2 part 2: {}", s);
}

// Game 1: 3 blue, 4 red;  1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn power_and_min() {
        let s = "Game 1: 3 blue, 4 red;  1 red, 2 green, 6 blue; 2 green";
        let g = parse_one_game(s);
        let min = g.min_cube();
        //4 red, 2 green, and 6 blue cubes
        assert_eq!(4, min.red);
        assert_eq!(2, min.green);
        assert_eq!(6, min.blue);
        assert_eq!(48, min.power());

        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let g = parse_one_game(s);
        let min = g.min_cube();
        //4 red, 2 green, and 6 blue cubes
        assert_eq!(20, min.red);
        assert_eq!(13, min.green);
        assert_eq!(6, min.blue);
        assert_eq!(1560, min.power());
    }

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
