use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

type Coord = (i32, i32, i32);

fn iterate(set: HashSet<Coord>) -> HashSet<Coord> {
    let mut v: Vec<Coord> = Vec::new();
    for (x, y, z) in set.iter() {
        v.append(
            &mut vec![x, y, z]
                .iter()
                .map(|&a| (a - 1..a + 2))
                .multi_cartesian_product()
                .map(|x| x.into_iter().collect_tuple().unwrap())
                .collect(),
        )
    }
    v.sort();
    let mut new_set: HashSet<Coord> = HashSet::new();
    for (c, i) in v
        .into_iter()
        .group_by(|&i| i)
        .into_iter()
        .map(|(a, b)| (a, b.count()))
    {
        match i {
            4 => {
                if set.contains(&c) {
                    new_set.insert(c);
                }
            }
            3 => {
                new_set.insert(c);
            }
            _ => {}
        }
    }
    new_set
}

type Coord4 = (i32, i32, i32, i32);

fn iterate4(set: HashSet<Coord4>) -> HashSet<Coord4> {
    let mut v: Vec<Coord4> = Vec::new();
    for (w, x, y, z) in set.iter() {
        v.append(
            &mut vec![w, x, y, z]
                .iter()
                .map(|&a| (a - 1..a + 2))
                .multi_cartesian_product()
                .map(|x| x.into_iter().collect_tuple().unwrap())
                .collect(),
        )
    }
    v.sort();
    let mut new_set: HashSet<Coord4> = HashSet::new();
    for (c, i) in v
        .into_iter()
        .group_by(|&i| i)
        .into_iter()
        .map(|(a, b)| (a, b.count()))
    {
        match i {
            4 => {
                if set.contains(&c) {
                    new_set.insert(c);
                }
            }
            3 => {
                new_set.insert(c);
            }
            _ => {}
        }
    }
    new_set
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut set: HashSet<Coord> = HashSet::new();

    for (n, line) in input.lines().enumerate() {
        for (m, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert((n as i32, m as i32, 0));
            }
        }
    }

    let set4: HashSet<Coord4> = set.iter().map(|&(x, y, z)| (x, y, z, 0)).collect();
    let part1 = iterate(iterate(iterate(iterate(iterate(iterate(set))))))
        .iter()
        .count();

    println!("Part 1: {}", part1);

    let part2 = iterate4(iterate4(iterate4(iterate4(iterate4(iterate4(set4))))))
        .iter()
        .count();

    println!("Part 2: {}", part2);
}
