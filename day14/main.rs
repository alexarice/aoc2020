use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone)]
struct Mask {
    to_one: u64,
    to_zero: u64,
}

fn create_mask(s: &str) -> Mask {
    let to_one = u64::from_str_radix(&(s.to_string().replace("X", "0")), 2).unwrap();
    let to_zero = u64::from_str_radix(&(s.to_string().replace("X", "1")), 2).unwrap();
    Mask { to_one, to_zero }
}

fn run_mask(i: u64, Mask { to_one, to_zero }: Mask) -> u64 {
    (i | to_one) & to_zero
}

fn create_mask_help(s: &String) -> Vec<Mask> {
    if s.contains("Y") {
        let s1 = s.clone().replacen("Y", "0", 1);
        let s2 = s.clone().replacen("Y", "1", 1);
        create_mask_help(&s1)
            .iter()
            .chain(create_mask_help(&s2).iter())
            .cloned()
            .collect()
    } else {
        vec![create_mask(s)]
    }
}

fn create_mask2(s: &str) -> Vec<Mask> {
    let s = s.to_string();
    create_mask_help(&s.replace("X", "Y").replace("0", "X"))
}

fn run_mask2(i: u64, v: &Vec<Mask>) -> Vec<u64> {
    v.iter().map(|&x| run_mask(i, x)).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let re = Regex::new(r"(?:mask = ([01X]{36}))|(?:mem\[(\d+)\] = (\d+))").unwrap();

    let lines = || input.lines();

    let mut mask = Mask {
        to_one: 0,
        to_zero: u64::MAX,
    };
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for l in lines() {
        let caps = re.captures(l).unwrap();
        match caps.get(2) {
            None => {
                mask = create_mask(&caps[1]);
            }
            Some(caps2) => {
                mem.insert(
                    caps2.as_str().parse().unwrap(),
                    run_mask((&caps[3]).parse().unwrap(), mask),
                );
            }
        };
    }

    println!("Part 1: {}", mem.values().sum::<u64>());

    let mut mask: Vec<Mask> = vec![];
    mem = HashMap::new();

    for l in lines() {
        let caps = re.captures(l).unwrap();
        match caps.get(2) {
            None => {
                mask = create_mask2(&caps[1]);
            }
            Some(caps2) => {
                for x in run_mask2(caps2.as_str().parse().unwrap(), &mask) {
                    mem.insert(x, caps[3].parse().unwrap());
                }
            }
        };
    }

    println!("Part 2: {}", mem.values().sum::<u64>());
}
