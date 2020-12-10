use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut i = 25;

    while i < numbers.len()
        && (&numbers[i - 25..i])
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == numbers[i])
    {
        i += 1;
    }

    println!("Part 1: {}", numbers[i]);
    let target = numbers[i];

    for (a, b) in (0..numbers.len()).tuple_combinations() {
        if a + 1 < b {
            let range = || (&numbers[a..b]).iter();
            if range().sum::<u64>() == target {
                println!(
                    "Part 2: {}",
                    range().min().unwrap() + range().max().unwrap()
                )
            }
        }
    }
}
