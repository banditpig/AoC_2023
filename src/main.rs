#![allow(non_snake_case)]

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
fn day2(){

}
fn main() {
    day1();
    day2()
}