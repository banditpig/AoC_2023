#![allow(non_snake_case)]
#![allow(unused)]
use crate::day2::{CubeSet, GameSet};
use std::thread;
use std::time::Instant;

mod day1;
mod day2;
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
}
fn day2() {}
fn main() {
    //day1();
    //day2()
    let gs = day2::parse_games();
    //12 red cubes, 13 green cubes, and 14 blue cubes?
    let game_set = GameSet { games: gs };
    let constraint = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let x = game_set.passing_games_sum(&constraint);
    dbg!(x);
}
