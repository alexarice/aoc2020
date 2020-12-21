use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = input.lines();

    let re = Regex::new(r"(.*) \(contains (.*)\)").unwrap();

    let parsed: Vec<(HashSet<String>, Vec<String>)> = lines
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let ingredients = caps[1].split(" ").map(|x| x.to_string()).collect();
            let allergens = caps[2].split(", ").map(|x| x.to_string()).collect();
            (ingredients, allergens)
        })
        .collect();

    let mut a_map: HashMap<&String, HashSet<String>> = HashMap::new();
    let mut i_map: HashMap<&String, usize> = HashMap::new();

    for (ingredients, allergens) in parsed.iter() {
        for allergen in allergens {
            if a_map.contains_key(allergen) {
                let int = &a_map[allergen] & &ingredients;
                a_map.insert(allergen, int);
            } else {
                a_map.insert(allergen, ingredients.clone());
            }
        }
        for i in ingredients {
            i_map.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let mut part1 = 0;

    for (i, n) in i_map {
        if a_map.values().all(|s| !s.contains(i)) {
            part1 += n;
        }
    }

    println!("Part 1: {}", part1);

    let mut dil: Vec<(String, String)> = vec![];

    while let Some(k) = a_map.keys().cloned().find(|k| a_map[k].len() == 1) {
        let val = a_map[k].iter().cloned().next().unwrap();
        a_map.remove(k);
        for (_, v) in a_map.iter_mut() {
            v.remove(&val);
        }
        dil.push((k.clone(), val));
    }

    dil.sort();

    let part2 = dil[1..]
        .into_iter()
        .fold((&dil[0].1).to_owned(), |acc, e| acc + "," + &e.1);

    println!("Part 2: {}", part2);
}
