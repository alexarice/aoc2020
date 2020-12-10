use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let numbers = input
        .lines()
        .map(|x| x.parse::<u32>().expect("Parsing failed"));

    let mut adapters: Vec<_> = numbers.collect();

    adapters.sort();

    let mut last = 0;
    let mut one_diff = 0;
    let mut three_diff = 0;

    for &x in &adapters {
        match x - last {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => (),
        }
        last = x;
    }
    three_diff += 1;

    println!("Part 1: {}", one_diff * three_diff);

    let mut map: HashMap<u32, u64> = HashMap::new();

    map.insert(adapters.pop().unwrap(), 1);

    let get = |m: &HashMap<u32, u64>, x: u32| {
        m.get(&(x + 1)).unwrap_or(&0)
            + m.get(&(x + 2)).unwrap_or(&0)
            + m.get(&(x + 3)).unwrap_or(&0)
    };

    adapters.reverse();
    for x in adapters {
        map.insert(x, get(&map, x));
    }

    let part2 = get(&map, 0);

    println!("Part 2: {}", part2)
}
