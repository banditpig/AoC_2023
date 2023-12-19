#![allow(non_snake_case)]
#![allow(unused)]

use std::thread;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod day10;
mod day11;
mod day11_2;
mod day8;
mod day9;
mod utils;

fn day1() {
    //Sequential
    let start_time = Instant::now();
    day1::part1();
    day1::part2();
    let end_time = Instant::now();
    let diff = end_time - start_time;
    println!("Elapsed time: {} ms", diff.as_millis());
    println!();

    //Thread per input string.
    //*slower* than sequential -  it spawns a thread for each string
    //and it's about half the time of sequential.
    let start_time = Instant::now();
    day1::part1_threaded();
    day1::part2_threaded();
    let end_time = Instant::now();
    let diff = end_time - start_time;
    println!("Elapsed time: {} ms", diff.as_millis());
    println!();

    //Runs part1 and part2 in parallel. This is fastest.
    let start_time = Instant::now();
    let mut handles = vec![];
    let h = thread::spawn(move || {
        day1::part1();
    });
    handles.push(h);
    let h = thread::spawn(move || {
        day1::part2();
    });
    handles.push(h);
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    let end_time = Instant::now();
    let diff = end_time - start_time;
    println!("Elapsed time: {} ms", diff.as_millis());
    println!();
}

fn day2() {
    day2::part1();
    day2::part2();
}
fn day3() {
    day3::part1();
    day3::part2();
}
fn day4() {
    day4::part1();
    day4::part2();
}
fn main() {
    // day1();
    // day2();
    // day3();
    // day4();
    // day5::parse_almanac();
    // day6();

    //day7::part2();
    // day8::part1();
    // day8::part2();
    // day10::part1();
    // day10::part2();
    day11::part1();
    day11::part2();
}

fn day6() {
    day6::part1();
    day6::part2();
}
