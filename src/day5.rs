use crate::utils;
use itertools::Itertools;
use load_file::load_str;
use nom::error::dbg_dmp;
use std::collections::HashMap;
use std::slice::from_ref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::scope;

// Define a trait with an associated type representing the inner type
trait AlmanacInt {
    type Inner;

    // Define a method that operates on the inner type
    fn get_inner(&self) -> Self::Inner;
}

// Implement the trait for any newtype wrapping usize
impl AlmanacInt for Seed {
    type Inner = usize;

    fn get_inner(&self) -> usize {
        self.0
    }
}

// fn main() {
//     let my_newtype_instance = MyNewtype(42);
//
//     // Call the method defined in the Newtype trait
//     let inner_value = my_newtype_instance.get_inner();
//
//     println!("Inner value: {}", inner_value);
// }

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Seed(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Soil(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Fertilizer(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Water(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Light(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Temperature(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Humidity(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct Location(usize);
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Range {
    dest: usize, //values
    src: usize,  // keys
    length: usize,
}

impl Range {
    pub fn src_to_dest(&self, s: usize) -> Option<usize> {
        if (self.src..self.src + self.length).contains(&s) {
            let ix = s - self.src;
            Some(self.dest + ix)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Block {
    name: String,
    ranges: Vec<Range>,
}

impl Block {
    pub fn src_to_dest(&self, src: usize) -> usize {
        for r in &self.ranges {
            if let Some(dest) = r.src_to_dest(src) {
                return dest;
            }
        }
        return src;
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<Seed>,

    blocks_map: HashMap<String, Block>,
}
//type CalibrationFunction = fn(&str) -> Vec<u32>;
type SeedParser = fn(&str) -> Vec<Seed>;
fn simple_seed_parser(s: &str) -> Vec<Seed> {
    s.split(' ')
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| Seed(s.parse::<usize>().unwrap()))
        .collect::<Vec<Seed>>()
}
fn pair_seed_parser(s: &str) -> Vec<Seed> {
    let seeds = simple_seed_parser(s);
    let mut all = vec![];
    for pair in seeds.chunks(2) {
        let s = pair[0].0;
        let len = pair[1].0;
        let v = (s..s + len)
            .into_iter()
            .map(|s| Seed(s))
            .collect::<Vec<Seed>>();
        all.extend(v);
    }

    all
}
pub fn build_almanac(seed_parser: SeedParser) -> (Vec<Seed>, HashMap<String, Block>) {
    let blocks = load_str!("../data/day5.txt")
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|b| parse_block(b))
        .collect::<Vec<_>>();

    let first = blocks.first().unwrap();
    let seeds = seed_parser(&*first.name.replace("seeds: ", ""));

    let mut blocks_map = HashMap::new();
    for b in blocks.into_iter() {
        blocks_map.insert(b.name.to_string(), b);
    }
    (seeds, blocks_map)
}

// fn extract_seeds(&self) {
//     let seed_block = self.blocks_map.get("seeds:").unwrap();
// }

fn seed_to_soil(blocks_map: &HashMap<String, Block>, s: &Seed) -> Soil {
    let b = blocks_map.get("seed-to-soil map:").unwrap();
    Soil(b.src_to_dest(s.0))
}

fn soil_to_fertilizer(blocks_map: &HashMap<String, Block>, s: &Soil) -> Fertilizer {
    let b = blocks_map.get("soil-to-fertilizer map:").unwrap();
    Fertilizer(b.src_to_dest(s.0))
}

fn fertilizer_to_water(blocks_map: &HashMap<String, Block>, f: &Fertilizer) -> Water {
    let b = blocks_map.get("fertilizer-to-water map:").unwrap();
    Water(b.src_to_dest(f.0))
}

fn water_to_light(blocks_map: &HashMap<String, Block>, w: &Water) -> Light {
    let b = blocks_map.get("water-to-light map:").unwrap();
    Light(b.src_to_dest(w.0))
}

fn light_to_temperature(blocks_map: &HashMap<String, Block>, l: &Light) -> Temperature {
    let b = blocks_map.get("light-to-temperature map:").unwrap();
    Temperature(b.src_to_dest(l.0))
}

fn temperature_to_humidity(blocks_map: &HashMap<String, Block>, t: &Temperature) -> Humidity {
    let b = blocks_map.get("temperature-to-humidity map:").unwrap();
    Humidity(b.src_to_dest(t.0))
}

fn humidity_to_location(blocks_map: &HashMap<String, Block>, h: &Humidity) -> Location {
    let b = blocks_map.get("humidity-to-location map:").unwrap();
    Location(b.src_to_dest(h.0))
}
fn route(blocks_map: &HashMap<String, Block>, s: &Seed) -> Location {
    //maybe find/make compostion function! :) https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust
    // humidity_to_location(
    //     temperature_to_humidity(
    //         light_to_temperature(
    //             water_to_light(
    //                 fertilizer_to_water(
    //                     soil_to_fertilizer(
    //                         seed_to_soil(s, blocks_map),blocks_map ),blocks_map),blocks_map
    //     ),blocks_map
    // )));

    let ss = seed_to_soil(blocks_map, s);
    let ss = soil_to_fertilizer(blocks_map, &ss);
    let ss = fertilizer_to_water(blocks_map, &ss);
    let ss = water_to_light(blocks_map, &ss);
    let ss = light_to_temperature(blocks_map, &ss);
    let ss = temperature_to_humidity(blocks_map, &ss);
    humidity_to_location(blocks_map, &ss)
}
//Vec<&'static str>
pub fn lowest_location(blocks_map: HashMap<String, Block>, seeds: Vec<Seed>) -> Location {
    let locs = Arc::new(Mutex::new(vec![]));
    let blocks_map = Arc::new(Mutex::new(blocks_map));
    let mut handles = vec![];

    for s in &seeds {
        let s = s.clone();
        let locs = Arc::clone(&locs);
        let blocks_map = Arc::clone(&blocks_map);
        let handle = thread::spawn(move || {
            let r = route(&blocks_map.lock().unwrap(), &s);

            locs.lock().unwrap().push(r);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    let binding = locs.lock().unwrap();
    // Now locs is outside the threads, and you can access the collected results
    let lowest = binding.iter().min().unwrap();
    *lowest
}

fn parse_range(r: &str) -> Range {
    //eg 10 32 8
    let r = r
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    Range {
        dest: *r.first().unwrap(),
        src: *r.iter().nth(1).unwrap(),
        length: *r.iter().nth(2).unwrap(),
    }
}

fn parse_block(b: &str) -> Block {
    let mut b = b.split('\n').collect::<Vec<&str>>();
    let head = b.remove(0).to_string();

    let ranges = b.iter().map(|l| parse_range(l)).collect::<Vec<_>>();

    Block { name: head, ranges }
}

pub fn parse_almanac() {
    let (seeds, blocks) = build_almanac(pair_seed_parser);

    let l = lowest_location(blocks, seeds);
    dbg!(l);
}
mod tests {
    use super::*;
    #[test]
    fn range_maps() {
        // 50 98 2
        // 52 50 48
        let mut r = Range {
            dest: 50,
            src: 98,
            length: 2,
        };
    }
    #[test]
    fn range() {
        let s = "23 3 44";
        let r = parse_range(s);
        assert_eq!(23, r.dest);
        assert_eq!(3, r.src);
        assert_eq!(44, r.length);
        //
        let s = "49 53 8";
        let r = parse_range(s);
        assert_eq!(49, r.dest);
        assert_eq!(53, r.src);
        assert_eq!(8, r.length);
    }
    #[test]
    fn block() {
        let b = "soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";
        let block = parse_block(b);
        dbg!(block);
    }
    #[test]
    fn parse_all() {
        parse_almanac();
    }
}
