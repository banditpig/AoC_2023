use crate::utils::load_input;
use std::sync::{Arc, Mutex};

type CalibrationFunction = fn(&str) -> Vec<u32>;

fn first_last(v: &Vec<u32>) -> u32 {
    match v.len() {
        1 => v.first().unwrap() * 10 + v.first().unwrap(),
        _ => v.first().unwrap() * 10 + v.last().unwrap(),
    }
}
fn replace_words_with_digit(s: &str) -> Vec<u32> {
    let l = s.len();
    let mut digits = vec![];
    for i in 0..l {
        let c = s.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap());
        } else {
            let part = s.get(i..);
            match part {
                None => {}
                Some(s) => {
                    if s.starts_with("one") {
                        digits.push(1)
                    }
                    if s.starts_with("two") {
                        digits.push(2)
                    }
                    if s.starts_with("three") {
                        digits.push(3)
                    }
                    if s.starts_with("four") {
                        digits.push(4)
                    }
                    if s.starts_with("five") {
                        digits.push(5)
                    }
                    if s.starts_with("six") {
                        digits.push(6)
                    }
                    if s.starts_with("seven") {
                        digits.push(7)
                    }
                    if s.starts_with("eight") {
                        digits.push(8)
                    }
                    if s.starts_with("nine") {
                        digits.push(9)
                    }
                }
            }
        }
    }
    digits
}

fn replace_chars_with_digits(s: &str) -> Vec<u32> {
    let mut digits = vec![];
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap());
        }
    }
    digits
}

fn apply_calibration(s: &str, f: CalibrationFunction) -> Vec<u32> {
    f(s)
}

use std::thread;
fn solve_threaded(v: Vec<&'static str>, f: CalibrationFunction) -> u32 {
    let mut handles = vec![];
    let sum = Arc::new(Mutex::new(0));
    for s in v {
        let sum_ = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let data = apply_calibration(s, f);
            let mut value = sum_.lock().unwrap();
            *value += first_last(&data);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    let x = *sum.lock().unwrap();
    x
}

fn solve(v: Vec<&str>, f: CalibrationFunction) -> u32 {
    v.iter().map(|s| first_last(&apply_calibration(s, f))).sum()
}

pub fn part1() {
    let v = load_input("../data/day1.txt");
    let r = solve(v, replace_chars_with_digits);
    println!("Part 1: {r}");
}
pub fn part2() {
    let v = load_input("../data/day1.txt");
    let r = solve(v, replace_words_with_digit);
    println!("Part 2: {r}");
}
pub fn part1_threaded() {
    let v = load_input("../data/day1.txt");
    let r = solve_threaded(v, replace_chars_with_digits);
    println!("Part 1: {r}");
}
pub fn part2_threaded() {
    let v = load_input("../data/day1.txt");
    let r = solve_threaded(v, replace_words_with_digit);
    println!("Part 2: {r}");
}
