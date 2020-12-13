use std::fs;

extern crate num;
use num::Integer;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines: Vec<_> = input.lines().collect();

    let timestamp = lines[0].parse::<u32>().unwrap();
    let ids: Vec<_> = lines[1]
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let part1 = ids
        .iter()
        .map(|&x| (x, x * (timestamp / x + 1) - timestamp))
        .min_by_key(|x| x.1)
        .unwrap();

    println!("Part 1: {}", part1.0 * part1.1);

    let ids_with_offset: Vec<_> = lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(n, x)| (n, x.parse::<u64>().unwrap()))
        .collect();

    let mut running_timestamp: u64 = 0;
    let mut running_interval: u64 = 1;

    for (n, x) in ids_with_offset {
        while (running_timestamp + (n as u64)) % x != 0 {
            running_timestamp += running_interval;
        }
        running_interval = running_interval.lcm(&x);
    }

    println!("Part 2: {}", running_timestamp)
}
