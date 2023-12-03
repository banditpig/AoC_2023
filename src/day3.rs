use crate::utils::load_input;

use std::collections::HashMap;

pub fn part1() {
    let lines = load_input("../data/day3.txt");
    let mut eng = Engine::new();
    eng.build_engine(lines);

    let mut res = vec![];
    for (pos, nmbr) in eng.numbers {
        let neighbours = nmbr.neighbours(&pos);
        if neighbours.iter().any(|pos| eng.symbols.contains_key(pos)) {
            res.push(nmbr);
        }
    }
    let sum = res.iter().fold(0, |acc, n| acc + n.value);
    println!("Day 3 Part 1: {}", sum);
}
#[derive(Hash, Eq, PartialEq, Default, Debug)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
#[derive(Hash, Eq, PartialEq, Default, Debug)]
struct Number {
    length: usize,
    value: usize,
}

impl Number {
    pub fn neighbours(&self, start: &Pos) -> Vec<Pos> {
        let mut n = vec![];
        let x = start.x;
        let y = start.y;
        let len = self.length - 1;
        for ix in 0..=self.length - 1 {
            if y > 0 {
                n.push(Pos::new(x + ix, y - 1));
            }
            n.push(Pos::new(x + ix, y));
            n.push(Pos::new(x + ix, y + 1));
        }
        if x > 0 {
            n.push(Pos::new(x - 1, y));
            if y > 0 {
                n.push(Pos::new(x - 1, y - 1));
            }
            n.push(Pos::new(x - 1, y + 1));
        }

        n.push(Pos::new(x + len + 1, y));
        if y > 0 {
            n.push(Pos::new(x + len + 1, y - 1));
        }
        n.push(Pos::new(x + len + 1, y + 1));

        n
    }
}

#[derive(Debug)]
struct Engine {
    numbers: HashMap<Pos, Number>,
    symbols: HashMap<Pos, char>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            numbers: HashMap::new(),
            symbols: HashMap::new(),
        }
    }
    pub fn build_engine(&mut self, lines: Vec<&str>) {
        //iy is each row
        for (row, line) in lines.iter().enumerate() {
            self.parse_line(row, line);
        }
    }
    fn parse_line(&mut self, row: usize, line: &str) {
        let mut col = 0usize;

        while col < line.len() {
            let c = line.chars().nth(col).unwrap();
            match c {
                '0'..='9' => {
                    let x = col;
                    let mut numbr = String::from(c);
                    col += 1;
                    while let Some(c) = line.chars().nth(col) {
                        if c.is_ascii_digit() {
                            numbr.push(line.chars().nth(col).unwrap());
                            col += 1;
                        } else {
                            break;
                        }
                    }
                    let n = numbr.parse::<usize>().unwrap();
                    let number = Number {
                        length: numbr.len(),
                        value: n,
                    };
                    self.numbers.insert(Pos { x, y: row }, number);
                }

                '.' => {
                    col += 1;
                }

                _ => {
                    self.symbols.insert(Pos { x: col, y: row }, c);
                    col += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
