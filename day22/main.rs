use primes::{PrimeSet, Sieve};
use std::collections::{HashSet, VecDeque};
use std::fs;

fn score(v: &VecDeque<usize>) -> usize {
    v.iter().enumerate().map(|(n, x)| x * (v.len() - n)).sum()
}

fn hash(p: &VecDeque<usize>, pset: &mut Sieve) -> usize {
    p.iter()
        .zip(pset.iter())
        .map(|(x, y)| x * (y as usize))
        .sum()
}

fn recursive_game(
    mut p1: VecDeque<usize>,
    mut p2: VecDeque<usize>,
    pset: &mut Sieve,
) -> (bool, VecDeque<usize>) {
    let mut set: HashSet<(usize, usize)> = HashSet::new();

    while !set.contains(&(hash(&p1, pset), hash(&p2, pset))) && p1.len() > 0 && p2.len() > 0 {
        set.insert((hash(&p1, pset), hash(&p2, pset)));
        let x = p1.pop_front().unwrap();
        let y = p2.pop_front().unwrap();
        let b: bool;
        if x <= p1.len() && y <= p2.len() {
            b = recursive_game(
                p1.iter().take(x).cloned().collect(),
                p2.iter().take(y).cloned().collect(),
                pset,
            )
            .0
        } else {
            b = x > y;
        }
        if b {
            p1.push_back(x);
            p1.push_back(y);
        } else {
            p2.push_back(y);
            p2.push_back(x);
        }
    }

    if p1.len() > 0 {
        (true, p1)
    } else {
        (false, p2)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let split: Vec<_> = input.split("\n\n").collect();

    let get_player = |n: usize| {
        split[n]
            .lines()
            .skip(1)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>()
    };

    let mut player1: VecDeque<_> = get_player(0);
    let mut player2: VecDeque<_> = get_player(1);

    while player1.len() > 0 && player2.len() > 0 {
        let x = player1.pop_front().unwrap();
        let y = player2.pop_front().unwrap();

        if x > y {
            player1.push_back(x);
            player1.push_back(y);
        } else {
            player2.push_back(y);
            player2.push_back(x);
        }
    }

    let part1 = [player1, player2].iter().map(score).max().unwrap();

    println!("Part 1: {}", part1);

    let part2 = score(&recursive_game(get_player(0), get_player(1), &mut Sieve::new()).1);

    println!("Part 2: {}", part2);
}
