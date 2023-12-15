use crate::utils::{lcm_of_numbers, load_input};
use itertools::fold;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Network {
    nodes: HashMap<String, (String, String)>,
    instr: Vec<char>,
}
impl Network {
    pub fn eval2(&self) -> Vec<u128> {
        let start_nodes = self.get_starting_nodes();
        let mut res = vec![];
        let mut prod: u128 = 1;
        for node in start_nodes {
            let l = self.eval(node, |n: &str| n.ends_with("Z")) as u128;
            res.push(l);
        }
        res
    }

    pub fn eval(&self, start: String, end_criteria: fn(&str) -> bool) -> usize {
        let mut found = false;
        let mut steps = 0;
        let mut current = &start;
        let mut instr = self.instr.get(steps % self.instr.len()).unwrap();
        while !found {
            let (l, r) = self.nodes.get(&*current).unwrap();
            steps += 1;
            match instr {
                'L' => {
                    current = l;
                    if end_criteria(current) {
                        found = true;
                    }
                }
                'R' => {
                    current = r;
                    if end_criteria(current) {
                        found = true;
                    }
                }
                _ => panic!("oops"),
            }
            instr = self.instr.get(steps % self.instr.len()).unwrap();
        }

        steps
    }
    pub fn get_starting_nodes(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        self.nodes.keys().fold(vec![], |mut acc, nxt| {
            if self.ends_with('A', nxt) {
                acc.push(nxt.clone());
            }
            acc
        })
    }
    fn step(&self, instr: &char, node: String) -> String {
        let (l, r) = self.nodes.get(&*node).unwrap();
        match instr {
            'L' => l.to_string(),
            'R' => r.to_string(),
            _ => panic!(),
        }
    }
    fn ends_with(&self, ch: char, node: &str) -> bool {
        node.ends_with(ch)
    }
}

fn parse_input() -> (Vec<char>, Vec<(String, String, String)>) {
    //Vec<(&'a str, &'a str, &'a str)> {
    let mut data = load_input("../data/day8.txt");
    let instr = data.remove(0).chars().collect::<Vec<_>>();
    data.remove(0);
    let mut inp = vec![];
    for line in data {
        let l = line.replace(" ", "").replace("(", "").replace(")", "");
        //AAA = (BBB, CCC)
        let (key, right) = l.split_once('=').unwrap();
        let (ll, rr) = right.split_once(",").unwrap();
        inp.push((key.to_owned(), ll.to_owned(), rr.to_owned()));
    }
    (instr, inp)
}

fn create_map(inp: Vec<(String, String, String)>) -> HashMap<String, (String, String)> {
    let mut map = HashMap::new();
    for (k, l, r) in inp {
        map.insert(k, (l, r));
    }
    map
}
pub fn part1() {
    let (instr, pairs) = parse_input();
    let nodes = create_map(pairs);

    let nw = Network { nodes, instr };
    let steps = nw.eval("AAA".to_string(), |n: &str| n.eq("ZZZ"));
    println!("Day 8. Part 1: {steps}");
}
pub fn part2() {
    let (instr, pairs) = parse_input();
    let nodes = create_map(pairs);

    let nw = Network { nodes, instr };
    let steps = nw.eval2();
    let x = lcm_of_numbers(steps);
    println!("Day 8. Part 2: {x}");
}
