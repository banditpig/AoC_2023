use crate::utils;
use itertools::Itertools;
use load_file::load_str;
use nom::error::dbg_dmp;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::ffi::c_ushort;
use std::slice::from_ref;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::scope;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
struct Range {
    dest: usize,
    src: usize,
    length: usize,
}

impl Range {
    pub fn src_to_dest(&self, s: usize) -> Option<usize> {
        if (s < self.src + self.length) && (s >= self.src) {
            Some(self.dest + (s - self.src))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Block {
    name: String,
    ranges: Vec<Range>,
}

impl Block {
    pub fn src_to_dest(&self, src: usize) -> usize {
        let larger = self
            .ranges
            .iter()
            .skip_while(|&item| item.src < src)
            .collect::<Vec<_>>();
        for r in larger {
            if let Some(dest) = r.src_to_dest(src) {
                return dest;
            }
        }
        return src;
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,

    blocks_map: BTreeMap<String, Block>,
}
//type CalibrationFunction = fn(&str) -> Vec<u32>;
type SeedParser = fn(&str) -> Vec<usize>;
fn simple_seed_parser(s: &str) -> Vec<usize> {
    s.split(' ')
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| (s.parse::<usize>().unwrap()))
        .collect::<Vec<usize>>()
}
fn pair_seed_parser(s: &str) -> Vec<usize> {
    let seeds = simple_seed_parser(s);
    let mut all = vec![];
    for pair in seeds.chunks(2) {
        let s = pair[0];
        let len = pair[1];
        let v = (s..s + len).into_iter().collect::<Vec<usize>>();
        all.extend(v);
    }

    all
}
pub fn build_almanac(seed_parser: SeedParser) -> (Vec<usize>, BTreeMap<String, Block>) {
    let blocks = load_str!("../data/day5.txt")
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|b| parse_block(b))
        .collect::<Vec<_>>();

    let first = blocks.first().unwrap();
    let seeds = seed_parser(&*first.name.replace("seeds: ", ""));

    let mut blocks_map = BTreeMap::new();
    for b in blocks.into_iter() {
        blocks_map.insert(b.name.to_string(), b);
    }
    (seeds, blocks_map)
}

fn routes(blocks_map: &BTreeMap<String, Block>, seeds: &[usize]) -> Vec<usize> {
    //===

    let seed_to_soil_b = blocks_map.get("seed-to-soil map:").unwrap();
    let soil_to_fertilizer_b = blocks_map.get("soil-to-fertilizer map:").unwrap();
    let fertilizer_to_water_b = blocks_map.get("fertilizer-to-water map:").unwrap();
    let water_to_light_b = blocks_map.get("water-to-light map:").unwrap();
    let light_to_temperature_b = blocks_map.get("light-to-temperature map:").unwrap();
    let temperature_to_humidity_b = blocks_map.get("temperature-to-humidity map:").unwrap();
    let humidity_to_location_b = blocks_map.get("humidity-to-location map:").unwrap();

    let mut results = vec![];
    for s in seeds {
        let s = seed_to_soil_b.src_to_dest(*s);

        let s = soil_to_fertilizer_b.src_to_dest(s);

        let s = fertilizer_to_water_b.src_to_dest(s);

        let s = water_to_light_b.src_to_dest(s);

        let s = light_to_temperature_b.src_to_dest(s);

        let s = temperature_to_humidity_b.src_to_dest(s);

        let s = humidity_to_location_b.src_to_dest(s);
        results.push(s);
    }
    //====
    results

    //seeds.iter().map(|s| route(blocks_map, s)).collect()
}
//Vec<&'static str>
pub fn lowest_location(blocks_map: BTreeMap<String, Block>, seeds: Vec<usize>) -> usize {
    //let locs = Arc::new(Mutex::new(vec![]));
    let blocks_map = Arc::new(Mutex::new(blocks_map));
    println!("Total seeds: {:?}", seeds.len()); //2658467274
                                                //100000000
    let my_atomic_int = Arc::new(AtomicI32::new(0));

    // Clone a reference to the atomic integer for each thread
    let thread_atomic_int = Arc::clone(&my_atomic_int);

    let ss = seeds
        .par_chunks(10)
        .flat_map(|chunk| {
            let start_time = Instant::now();
            thread_atomic_int.fetch_add(1, Ordering::SeqCst);
            println!("Thread start: {:?}", thread_atomic_int);

            let blocks_map = Arc::clone(&blocks_map);
            let x = routes(&blocks_map.lock().unwrap(), chunk);
            let elapsed_time = start_time.elapsed();
            println!("Elapsed time: {:?}", elapsed_time);
            x
        })
        .collect::<Vec<usize>>();
    *ss.iter().min().unwrap()
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

    let mut ranges = b.iter().map(|l| parse_range(l)).collect::<Vec<_>>();

    // ranges.sort_by(|b1, b2| b1.src.cmp(&b2.src));

    Block { name: head, ranges }
}

pub fn parse_almanac() {
    let (seeds, blocks) = build_almanac(simple_seed_parser);

    let l = lowest_location(blocks, seeds);
    dbg!(l);
}
mod tests {
    use super::*;
    #[test]
    fn range_maps() {
        // 50 98 2
        // 52 50 48
        let mut _r = Range {
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
