use std::arch::aarch64::veor_s8;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

struct Image {
    image: Vec<Vec<char>>,
    max_col: usize,
    max_row: usize,
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
            self.image.insert(ix + prior, vec!['.'; self.max_col]);
        }
    }

    fn expand_columns(&mut self) {
        //take vertical slice
        let mut col_ixs = vec![];
        for col in (0..self.max_col) {
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
        max_col,
        max_row,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn load() {
        let mut g = load_input();

        println!("{}", g);
        println!();
        g.expand();

        println!("{}", g);
    }
}
