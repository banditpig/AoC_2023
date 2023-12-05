use std::collections::HashMap;

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
    dest: usize,
    src: usize,
    length: usize,
}
#[derive(Debug)]
struct Block {
    name: String,
    ranges: Vec<Range>,
}
#[derive(Debug)]
struct Almanac {
    seeds: Vec<Seed>,
    location: HashMap<Humidity, Location>,
    humidity: HashMap<Temperature, Humidity>,
    temperature: HashMap<Light, Temperature>,
    light: HashMap<Water, Light>,
    water: HashMap<Fertilizer, Water>,
    fertilizer: HashMap<Soil, Fertilizer>,
    soil: HashMap<Seed, Soil>,
}
impl Almanac {
    fn seed_to_soil(&self, s: Seed) -> Soil {
        let v = self.soil.get(&s);
        match v {
            Some(v) => *v,
            None => Soil(s.0),
        }
    }

    fn soil_to_fertilizer(&self, s: Soil) -> Fertilizer {
        let v = self.fertilizer.get(&s);
        match v {
            Some(v) => *v,
            None => Fertilizer(s.0),
        }
    }

    fn fertilizer_to_water(&self, f: Fertilizer) -> Water {
        let v = self.water.get(&f);
        match v {
            Some(v) => *v,
            None => Water(f.0),
        }
    }

    fn water_to_light(&self, w: Water) -> Light {
        let v = self.light.get(&w);
        match v {
            Some(v) => *v,
            None => Light(w.0),
        }
    }

    fn light_to_temperature(&self, l: Light) -> Temperature {
        let v = self.temperature.get(&l);
        match v {
            Some(v) => *v,
            None => Temperature(l.0),
        }
    }

    fn temperature_to_humidity(&self, t: Temperature) -> Humidity {
        let v = self.humidity.get(&t);
        match v {
            Some(v) => *v,
            None => Humidity(t.0),
        }
    }

    fn humidity_to_location(&self, h: Humidity) -> Location {
        let v = self.location.get(&h);
        match v {
            Some(v) => *v,
            None => Location(h.0),
        }
    }
    fn route(&self, s: Seed) -> Location {
        //maybe find/make compostion function! :) https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust
        self.humidity_to_location(self.temperature_to_humidity(self.light_to_temperature(
            self.water_to_light(
                self.fertilizer_to_water(self.soil_to_fertilizer(self.seed_to_soil(s))),
            ),
        )))
    }
    pub fn lowest_location(&self) -> Location {
        let binding = self
            .seeds
            .clone()
            .into_iter()
            .map(|s| self.route(s))
            .collect::<Vec<_>>();

        let m = binding.iter().min().unwrap();
        *m
    }
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
mod tests {
    use super::*;

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
}
