use crate::utils::load_input;

fn diffs(nmbrs: Vec<isize>) -> Vec<isize> {
    nmbrs.windows(2).fold(vec![], |mut acc, w| {
        acc.push(w[1] - w[0]);
        acc
    })
}
fn all_diffs(mut nmbrs: Vec<isize>) -> Vec<Vec<isize>> {
    let mut r = vec![];
    let mut done = false;
    while !done {
        r.push(nmbrs.clone());
        nmbrs = diffs(nmbrs);
        done = nmbrs.iter().all(|n| *n == 0);
    }
    r
}
fn input() -> Vec<Vec<isize>> {
    let mut lines = load_input("../data/day9.txt");
    let mut res = vec![];
    for l in lines {
        let nmbrs = l
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        res.push(nmbrs);
    }
    res
}
fn next_val(diffs: Vec<Vec<isize>>) -> isize {
    diffs
        .iter()
        .fold(0, |mut acc, nxt| acc + nxt.last().unwrap())
}

fn next_val_front(mut diffs: Vec<Vec<isize>>) -> isize {
    diffs.reverse();
    let mut n = *diffs.remove(0).first().unwrap();

    diffs
        .iter()
        .fold(n, |mut acc, nxt| nxt.first().unwrap() - acc)
}

pub fn part1() {
    let input = input();
    let mut total = 0;
    for inp in input {
        let diffs = all_diffs(inp);
        let n = next_val(diffs);
        total += n;
    }
    println!("{total}");
}

pub fn part2() {
    let input = input();
    let mut total = 0;
    for inp in input {
        let diffs = all_diffs(inp.clone());
        let nv = next_val_front(diffs);
        let first = &inp.first().unwrap();
        total += nv;
    }
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        part1();
    }
    #[test]
    fn p2() {
        part2();
    }

    #[test]
    fn diff() {
        let n = vec![10, 13, 16, 21, 30, 45];
        println!("{:?}", diffs(n));
        let n1 = vec![3, 3, 5, 9, 15];
        println!("{:?}", diffs(n1));

        let n2 = vec![0, 2, 4, 6];
        println!("{:?}", diffs(n2));
        let n3 = vec![2, 2, 2];
        println!("{:?}", diffs(n3));

        let nall = vec![10, 13, 16, 21, 30, 45];
        println!("{:?}", all_diffs(nall));
    }
}
