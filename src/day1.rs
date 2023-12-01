
use crate::utils::load_input;

fn first_last(v: &Vec<u32>) -> u32{
    match v.len() {
        1 => v.first().unwrap() *10 + v.first().unwrap(),
        _ => v.first().unwrap() *10 + v.last().unwrap()
    }
}
fn replace_words_with_digit(s: &str) -> Vec<u32>{
    let l = s.len();
    let mut digits = vec![];
    for i in 0..l{
        let c  =s.chars().nth(i).unwrap();
        if c.is_ascii_digit(){
            digits.push(c.to_digit(10).unwrap());
        }else{
            let part = s.get(i..);
            match part {
                None => {}
                Some(s) => {
                    if  s.starts_with("one"){
                        digits.push(1)
                    }
                    if  s.starts_with("two"){
                        digits.push(2)
                    }
                    if  s.starts_with("three"){
                        digits.push(3)
                    }
                    if  s.starts_with("four"){
                        digits.push(4)
                    }
                    if  s.starts_with("five"){
                        digits.push(5)
                    }
                    if  s.starts_with("six"){
                        digits.push(6)
                    }
                    if  s.starts_with("seven"){
                        digits.push(7)
                    }
                    if  s.starts_with("eight"){
                        digits.push(8)
                    }
                    if  s.starts_with("nine"){
                        digits.push(9)
                    }
                }
            }
        }
    }
    digits
}

pub fn part2(){
    let v = load_input("../data/day1.txt")
        .iter()
        .map(|s| replace_words_with_digit(s))
        .collect::<Vec<Vec<u32>>>();

    let mut sum :u32 = 0;
    for s in &v{
        sum += first_last(s);
    }
    println!("Part 2: {sum}");
}

pub fn part1(){
    let v = load_input("../data/day1.txt");

    let mut sum :u32 = 0;
    for s in v {
        let nmbrs = s.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>()
            .iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        sum += first_last(&nmbrs);
    }
    println!("Part 1: {sum}");
}

