use itertools::Itertools;
use load_file::load_str;
use std::collections::HashMap;
use std::process::Termination;

pub fn load_input(file: &str) -> Vec<&'static str> {
    let v = load_str!(file).split('\n').collect::<Vec<&str>>();
    v
}
pub fn group_chars(chars: &str) -> String {
    let mut map = chars.chars().fold(HashMap::new(), |mut acc, nxt| {
        acc.entry(nxt).and_modify(|v| *v += 1).or_insert(1);
        acc
    });
    let mut tuple_vec = map.iter().fold(vec![], |mut acc, (k, v)| {
        acc.push((*k, *v));
        acc
    });

    tuple_vec.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    let mut out_vec = tuple_vec.iter().fold(Vec::new(), |mut acc, (c, cnt)| {
        (0..*cnt).for_each(|_ix| acc.push(*c));
        acc
    });
    let s: String = out_vec.iter().collect();
    s
}
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

pub fn lcm_of_numbers(numbers: Vec<u128>) -> u128 {
    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }
    result
}
