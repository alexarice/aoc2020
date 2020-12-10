extern crate nom;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};
use std::collections::HashMap;
use std::fs;

type Bag = String;

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |x: &str| x.parse::<u32>())(input)
}

fn bag_parser(input: &str) -> IResult<&str, Bag> {
    map(
        terminated(take_until(" bag"), pair(tag(" bag"), opt(tag("s")))),
        |x: &str| x.to_string(),
    )(input)
}

fn numbered_bag_parser(input: &str) -> IResult<&str, (Bag, u32)> {
    map(separated_pair(number, tag(" "), bag_parser), |(n, b)| {
        (b, n)
    })(input)
}

fn second_parser(input: &str) -> IResult<&str, HashMap<Bag, u32>> {
    terminated(
        alt((
            map(tag("no other bags"), |_| HashMap::new()),
            map(separated_list1(tag(", "), numbered_bag_parser), |x| {
                x.iter().cloned().collect::<HashMap<_, _>>()
            }),
        )),
        tag("."),
    )(input)
}

fn line_parser(input: &str) -> IResult<&str, (Bag, HashMap<Bag, u32>)> {
    separated_pair(bag_parser, tag(" contain "), second_parser)(input)
}

fn can_have_gold(dict: &HashMap<Bag, HashMap<Bag, u32>>, b: &Bag) -> bool {
    if let Some(x) = dict.get(b) {
        x.keys()
            .any(|y| y == "shiny gold" || can_have_gold(dict, y))
    } else {
        false
    }
}

fn bags_contained(dict: &HashMap<Bag, HashMap<Bag, u32>>, b: &Bag) -> u32 {
    let bags_inside = dict.get(b).unwrap().iter();

    bags_inside
        .map(|(bag, number)| (1 + bags_contained(dict, bag)) * number)
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = || input.lines().filter(|x| x.len() > 0);

    let dict: HashMap<Bag, HashMap<Bag, u32>> = lines()
        .map(|x| line_parser(x).expect("Bit of a yikes").1)
        .collect();

    println!("{}", dict.len());

    println!(
        "Part 1: {}",
        dict.keys().filter(|x| can_have_gold(&dict, x)).count()
    );

    println!(
        "Part 2: {}",
        bags_contained(&dict, &"shiny gold".to_string())
    )
}
