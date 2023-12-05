use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

struct Seed(usize);
struct Soil(usize);
struct Fertilizer(usize);
struct Water(usize);
struct Light(usize);
struct Temperature(usize);
struct Humidity(usize);
struct Location(usize);

fn route(s: Seed) -> Location {
    //maybe find/make compostion function! :) https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust
    humidity_to_location(temperature_to_humidity(light_to_temperature(
        water_to_light(fertilizer_to_water(soil_to_fertilizer(seed_to_soil(s)))),
    )))
}
fn seed_to_soil(s: Seed) -> Soil {
    todo!();
}

fn soil_to_fertilizer(s: Soil) -> Fertilizer {
    todo!();
}

fn fertilizer_to_water(f: Fertilizer) -> Water {
    todo!();
}

fn water_to_light(w: Water) -> Light {
    todo!();
}

fn light_to_temperature(l: Light) -> Temperature {
    todo!();
}

fn temperature_to_humidity(t: Temperature) -> Humidity {
    todo!();
}

fn humidity_to_location(h: Humidity) -> Location {
    todo!();
}
fn parse_range(r: &str) -> (usize, usize, usize) {
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
    (
        *r.first().unwrap(),
        *r.iter().nth(1).unwrap(),
        *r.iter().nth(2).unwrap(),
    )
}

fn parse_block(b: &Vec<&str>) -> Vec<(usize, usize, usize)> {
    todo!();
}
mod tests {
    use super::*;

    #[test]
    fn range() {
        let s = "23 3 44";
        let (a, b, c) = parse_range(s);
        assert_eq!(23, a);
        assert_eq!(3, b);
        assert_eq!(44, c);
        //
        let s = "49 53 8";
        let (a, b, c) = parse_range(s);
        assert_eq!(49, a);
        assert_eq!(53, b);
        assert_eq!(8, c);
    }
}
