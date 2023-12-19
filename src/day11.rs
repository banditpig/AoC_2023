use num_traits::abs;
use std::arch::aarch64::veor_s8;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use strum_macros::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn point(pair: (usize, usize)) -> Self {
        Point {
            x: pair.0,
            y: pair.1,
        }
    }
}
struct Image {
    image: Vec<Vec<char>>,
    maxx: usize,
    maxy: usize,
    map: HashMap<Point, char>,
    galaxies: Vec<Point>,
}
impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.image {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Image {
    pub fn expand(&mut self) {
        self.expand_rows();
        self.expand_columns();
        self.maxy = self.image.len();
        self.maxx = self.image.first().unwrap().len();
    }
    pub fn create_map(&mut self) {
        let mut map = HashMap::new();
        let mut galaxies = vec![];
        for (iy, row) in self.image.iter().enumerate() {
            for (ix, ch) in row.iter().enumerate() {
                let p = Point::point((ix, iy));
                map.insert(p, *ch);
                if (*ch == '#') {
                    galaxies.push(p);
                }
            }
        }
        self.galaxies = galaxies;
        self.map = map;
    }

    fn expand_rows(&mut self) {
        let mut ixs = vec![];
        for (ix, r) in self.image.iter().enumerate() {
            if r.iter().all(|ch| *ch == '.') {
                ixs.push(ix);
            }
        }
        let mut prior = 0;
        for (prior, ix) in ixs.iter().enumerate() {
            self.image.insert(ix + prior, vec!['.'; self.maxx]);
        }
    }

    fn expand_columns(&mut self) {
        //take vertical slice
        let mut col_ixs = vec![];
        for col in (0..self.maxx) {
            let mut temp = vec![];
            for row in &self.image {
                temp.push(row.get(col).unwrap())
            }
            if temp.iter().all(|c| **c == '.') {
                col_ixs.push(col);
            }
        }
        let mut prior = 0;
        for (prior, col_ix) in col_ixs.iter().enumerate() {
            for row in &mut self.image {
                row.insert(col_ix + prior, '.');
            }
        }
    }

    pub fn distance(&self, p1: Point, p2: Point) -> usize {
        let (x1, y1) = (p1.x, p1.y);
        let (x2, y2) = (p2.x, p2.y);

        x1.abs_diff(x2) + y1.abs_diff(y2)
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
        //todo!()
    }
}

fn load_input() -> Image {
    let mut lines = crate::utils::load_input("../data/day11.txt");
    let max_col = lines.first().unwrap().len();
    let max_row = lines.len();
    let mut image = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            row.push(c);
        }
        image.push(row);
    }

    Image {
        image,
        maxx: max_col,
        maxy: max_row,
        galaxies: vec![],
        map: HashMap::new(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::sync::{Arc, Mutex};
    use std::thread;
    #[test]
    fn galaxy_pairs() {
        let mut g = load_input();

        println!();
        g.expand();
        g.create_map();
        let pairs = g.pair_galaxies();
        let mut sum = 0;

        for (s, d) in pairs {
            let d = sum += g.distance(s, d);
        }

        println!("{:?}", sum);
    }
    //(1,6) (5,11)
}
