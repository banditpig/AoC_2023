// | is a vertical pipe connecting north and south. NS
// - is a horizontal pipe connecting east and west.EW
// L is a 90-degree bend connecting north and east. NE
// J is a 90-degree bend connecting north and west. NW
// 7 is a 90-degree bend connecting south and west.SW
// F is a 90-degree bend connecting south and east. SE
// . is ground; there is no pipe in this tile. G
//S is the starting position of the animal; there is a pipe on this tile,
// but your sketch doesn't show what shape the pipe has.
// -L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF

use crate::day10::Direction::{E, N, S, W};
use crate::day10::Pipe::{EW, GR, NE, NS, NW, SE, ST, SW};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

struct Grid {
    pub maxx: usize,
    pub maxy: usize,
    pub sx: usize,
    pub sy: usize,
    pipes: HashMap<(usize, usize), Pipe>,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, Display)]
enum Direction {
    N,
    S,
    W,
    E,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Display)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GR,
    ST,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.maxy) {
            for x in (0..self.maxx) {
                let p = *self.pipes.get(&(x, y)).unwrap();
                let ch = self.pipe_to_char(p);
                write!(f, "{p} {ch} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn char_to_pipe(c: char) -> Pipe {
    use Pipe::*;
    match c {
        '|' => NS,
        '-' => EW,
        'L' => SE,
        'J' => SW,
        '7' => NW,
        'F' => NE,
        '.' => GR,
        'S' => ST,
        _ => panic!(),
    }
}
impl Grid {
    fn pipe_to_char(&self, p: Pipe) -> char {
        use Pipe::*;
        match p {
            NS => '|',
            EW => '-',
            SE => 'L',
            SW => 'J',
            NW => '7',
            NE => 'F',
            GR => '.',
            Start => 'S',
            _ => panic!(),
        }
    }
    fn next_direction(&self, d: Direction, p: Pipe) -> Option<Direction> {
        use Direction::*;
        match p {
            // NS => '|',
            NS => match d {
                Direction::N => Some(N),
                Direction::S => Some(S),
                Direction::W => None,
                Direction::E => None,
            },
            EW => match d {
                // EW => '-',
                N => None,
                S => None,
                W => Some(W),
                E => Some(E),
            },
            NE => match d {
                //NE => 'F',
                N => Some(E),
                S => None,
                W => Some(S),
                E => None,
            },
            NW => match d {
                // NW => '7',
                N => Some(W),
                S => None,
                W => None,
                E => Some(S),
            },
            // SW => 'J',
            SW => match d {
                N => None,
                S => Some(W),
                W => None,
                E => Some(N),
            },
            SE => match d {
                // SE => 'L',
                N => None,
                S => Some(E),
                W => Some(N),
                E => None,
            },

            GR => match d {
                N => None,
                S => None,
                W => None,
                E => None,
            },
            Start => match d {
                N => None,
                S => None,
                W => None,
                E => None,
            },
        }
    }

    fn incx(&self, x: usize) -> usize {
        if x == self.maxx - 1 {
            x
        } else {
            x + 1
        }
    }
    fn decx(&self, x: usize) -> usize {
        if x == 0 {
            x
        } else {
            x - 1
        }
    }
    fn incy(&self, y: usize) -> usize {
        if y == self.maxy - 1 {
            y
        } else {
            y + 1
        }
    }
    fn decy(&self, y: usize) -> usize {
        if y == 0 {
            y
        } else {
            y - 1
        }
    }
    pub fn initial_directions(&self) -> Vec<Direction> {
        let mut res = vec![];

        let n = self.pipes.get(&(self.sx, self.decy(self.sy))).unwrap();
        let s = self.pipes.get(&(self.sx, self.incy(self.sy))).unwrap();
        let w = self.pipes.get(&(self.decx(self.sx), self.sy)).unwrap();
        let e = self.pipes.get(&(self.incx(self.sx), self.sy)).unwrap();

        if *n == NS || *n == NE || *n == NW {
            res.push(N);
        }
        if *s == NS || *s == SW || *s == SE {
            res.push(S);
        }
        if *w == EW || *w == NE || *w == SE {
            res.push(W);
        }
        if *e == EW || *e == NW || *e == SW {
            res.push(E);
        }

        res
    }
    pub fn next_location(&self, x: usize, y: usize, d: Direction) -> (usize, usize) {
        match d {
            N => (x, y - 1),
            S => (x, y + 1),
            W => (x - 1, y),
            E => (x + 1, y),
        }
    }
    pub fn follow_route(&self, mut dir: Direction) -> Vec<(usize, usize)> {
        let (mut x, mut y) = (self.sx, self.sy);

        let mut steps = vec![];
        let mut step = 1;
        let mut done = false;

        while !done {
            (x, y) = self.next_location(x, y, dir);
            let pipe = self.pipes.get(&(x, y)).unwrap();

            match self.next_direction(dir, *pipe) {
                None => done = true,
                Some(d) => dir = d,
            }
            steps.push((x, y));
            step += 1;
        }
        steps
    }
}

fn load_input() -> Grid {
    let mut lines = crate::utils::load_input("../data/day10.txt");
    let maxx = lines.first().unwrap().len();
    let maxy = lines.len();
    let mut x = 0;
    let mut y = 0;
    let mut sx = 0;
    let mut sy = 0;
    let mut pipes = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = char_to_pipe(c);
            if p == Pipe::ST {
                sx = x;
                sy = y
            }
            pipes.insert((x, y), p);
        }
    }
    Grid {
        maxx,
        maxy,
        pipes,
        sx,
        sy,
    }
}
pub fn part1() {
    let g = load_input();

    let init = g.initial_directions();
    let s1 = g.follow_route(*init.first().unwrap());
    let s2 = g.follow_route(*init.last().unwrap());
    for (ix, n) in s1.iter().enumerate() {
        if s2.get(ix).unwrap().eq(n) {
            println!("Day 10. Part 1: {}", ix + 1);
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn load() {
        let g = load_input();

        println!("{}", g);
    }
    #[test]
    fn initial_start_direction() {
        let g = load_input();
        println!("{}", g);
        let init = g.initial_directions();
        assert_eq!(2, init.len());
        assert!(init.contains(&S));
        assert!(init.contains(&E));
    }
    #[test]
    fn two_loops() {
        let g = load_input();

        let init = g.initial_directions();
        let s1 = g.follow_route(*init.first().unwrap());
        let s2 = g.follow_route(*init.last().unwrap());
        for (ix, n) in s1.iter().enumerate() {
            if s2.get(ix).unwrap().eq(n) {
                println!("====== {}", ix + 1);
                break;
            }
        }
        // println!("{:?}", s1);
        // println!("{:?}", s2);
    }
    #[test]
    fn try_loop() {
        let g = load_input();
        println!("{}", g);
        let init = g.initial_directions();

        //let mut dir = *init.first().unwrap();
        for mut dir in init {
            let (mut x, mut y) = (g.sx, g.sy);

            let mut step = 0;
            let mut done = false;
            while !done {
                (x, y) = g.next_location(x, y, dir);
                let pipe = g.pipes.get(&(x, y)).unwrap();
                println!("{}", g.pipe_to_char(*pipe));

                //dir = g.next_direction(dir, *pipe).unwrap();
                match g.next_direction(dir, *pipe) {
                    None => done = true,
                    Some(d) => dir = d,
                }
            }
            println!("------------------");
        }
    }
}
