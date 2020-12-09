extern crate regex;

use regex::Regex;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let mut counter = 0;
    let mut counter2 = 0;

    for cap in re.captures_iter(&input[..]) {
        let lower: usize = (&cap[1]).parse().unwrap();
        let upper: usize = (&cap[2]).parse().unwrap();
        let character: char = (&cap[3]).parse().unwrap();
        let c = (&cap[4]).chars().filter(|&x| x == character).count();
        if c >= lower && c <= upper {
            counter += 1;
        }
        let pw: Vec<char> = (&cap[4]).chars().collect();
        if pw[lower - 1] == character {
            if pw[upper - 1] != character {
                counter2 += 1;
            }
        } else if pw[upper - 1] == character {
            counter2 += 1;
        }
    }
    println!("Part 1 Count: {}", counter);
    println!("Part 2 Count: {}", counter2);
}
