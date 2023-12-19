use nom::ErrorConvert;
use num_traits::abs;
use std::arch::aarch64::veor_s8;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use strum_macros::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    pub fn point(pair: (i64, i64)) -> Self {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}
struct Image {
    maxx: i64,
    maxy: i64,
    empty_rows: Vec<i64>,
    empty_cols: Vec<i64>,
    map: HashMap<Point, char>,
    galaxies: Vec<Point>,
}
impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in (0..self.maxy) {
            for col in (0..self.maxx) {
                let p = Point::point((col, row));
                write!(f, "{}", self.map.get(&p).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Image {
    pub fn distance(&self, p1: Point, p2: Point, expansion: i64) -> i64 {
        // for x1, x2 how many expanded columns are there between x1 and x2
        // for y1, y2 how many expanded columns are there between y1 and y2
        let expansion = expansion - 1;
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);

        let sx = std::cmp::min(x1, x2);
        let ex = std::cmp::max(x1, x2);

        let sy = std::cmp::min(y1, y2);
        let ey = std::cmp::max(y1, y2);

        let mut expand_x = self
            .empty_cols
            .iter()
            .filter(|x| x < &&ex && x > &&sx)
            .collect::<Vec<_>>()
            .len() as i64;

        let mut expand_y = self
            .empty_rows
            .iter()
            .filter(|y| y < &&ey && y > &&sy)
            .collect::<Vec<_>>()
            .len() as i64;

        (sx.abs_diff(ex + expand_x * expansion) + sy.abs_diff(ey + expand_y * expansion)) as i64
    }
    pub fn pair_galaxies(&self) -> Vec<(Point, Point)> {
        let mut pairs = vec![];
        let mut ix = 1;
        let mut current_g = self.galaxies.get(0).unwrap();
        for pointer in (ix..self.galaxies.len()) {
            for ixx in (pointer..self.galaxies.len()) {
                let p = (*current_g, *self.galaxies.get(ixx).unwrap());
                pairs.push(p);
            }
            current_g = self.galaxies.get(ix).unwrap();
            ix += 1;
        }
        pairs
    }
}

fn load_input() -> Image {
    let mut map = HashMap::new();
    let mut galaxies = vec![];

    let mut lines = crate::utils::load_input("../data/day11.txt");
    let max_col = lines.first().unwrap().len() as i64;
    let max_row = lines.len() as i64;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point::point((x as i64, y as i64));
            map.insert(p, c);
            if c == '#' {
                galaxies.push(p);
            }
        }
    }
    //empty rows
    let mut empty_rows = vec![];
    for y in (0..max_row) {
        let mut empty = true;
        for x in (0..max_col) {
            let p = Point::point((x as i64, y as i64));
            if *map.get(&p).unwrap() == '#' {
                empty = false;
            }
        }
        if empty {
            empty_rows.push(y);
        }
    }
    let mut empty_cols = vec![];
    for x in (0..max_col) {
        let mut empty = true;
        for y in (0..max_row) {
            let p = Point::point((x as i64, y as i64));
            if *map.get(&p).unwrap() == '#' {
                empty = false;
            }
        }
        if empty {
            empty_cols.push(x);
        }
    }
    empty_rows.sort();
    empty_cols.sort();

    Image {
        maxx: max_col,
        maxy: max_row,
        galaxies,
        map,
        empty_rows,
        empty_cols,
    }
}

pub fn part1() {
    let mut g = load_input();

    let pairs = g.pair_galaxies();
    let mut sum = 0;

    for (s, d) in pairs {
        sum += g.distance(s, d, 2);
    }
    println!("Day 11. Part1: {}", sum);
}
pub fn part2() {
    let mut g = load_input();

    let pairs = g.pair_galaxies();
    let mut sum = 0;

    for (s, d) in pairs {
        sum += g.distance(s, d, 1_000_000); //904633799472
    }
    println!("Day 11. Part2: {}", sum);
}
