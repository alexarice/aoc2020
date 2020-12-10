use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers = input
        .lines()
        .map(|x| x.parse::<u32>().expect("Parsing failed"));

    for (i, j) in numbers.clone().tuple_combinations() {
        if i + j == 2020 {
            println!("{} * {} = {}", i, j, i * j);
        }
    }

    for (i, j, k) in numbers.tuple_combinations() {
        if i + j + k == 2020 {
            println!("{} * {} * {} = {}", i, j, k, i * j * k);
        }
    }
}
