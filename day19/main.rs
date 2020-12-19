use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Rule {
    Letter(char),
    Compound(Vec<Vec<u32>>),
}

type RuleList = HashMap<u32, Rule>;

use Rule::*;

fn parse_rule<'a>(r: &'a Rule, xs: &'a RuleList) -> impl Fn(&'a str) -> IResult<&'a str, ()> {
    let concat = move |v: &'a Vec<u32>| {
        move |i| {
            if v.len() == 1 {
                parse_rule(xs.get(&v[0]).unwrap(), xs)(i)
            } else {
                value(
                    (),
                    tuple((
                        parse_rule(xs.get(&v[0]).unwrap(), xs),
                        parse_rule(xs.get(&v[1]).unwrap(), xs),
                    )),
                )(i)
            }
        }
    };
    let r2 = r.clone();
    move |input| match r2 {
        Letter(c) => value((), char(*c))(input),
        Compound(v) => {
            if v.len() == 1 {
                concat(&v[0])(input)
            } else {
                alt((concat(&v[0]), concat(&v[1])))(input)
            }
        }
    }
}

fn parse<'a>(xs: &'a RuleList) -> impl Fn(&'a str) -> IResult<&'a str, ()> {
    move |input| all_consuming(parse_rule(xs.get(&0).unwrap(), xs))(input)
}

fn parse2<'a>(xs: &'a RuleList) -> impl Fn(&'a str) -> IResult<&'a str, bool> {
    move |input| {
        let (input, n) =
            fold_many1(parse_rule(xs.get(&42).unwrap(), xs), 0, |acc, _| acc + 1)(input)?;
        let (input, m) = all_consuming(fold_many1(
            parse_rule(xs.get(&31).unwrap(), xs),
            0,
            |acc, _| acc + 1,
        ))(input)?;
        println!("{} > {}", n, m);
        Ok((input, n > m))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let split_input: Vec<_> = input.split("\n\n").collect();

    let lines = || split_input[0].lines();

    let mut rule_list: HashMap<u32, Rule> = HashMap::new();

    let re = Regex::new(r#"(?P<index>\d+): (?:"(?P<letter>[ab])"|(?P<rule>.*))"#).unwrap();

    for l in lines() {
        let caps = re.captures(l).unwrap();
        let rule_no = caps["index"].parse::<u32>().unwrap();
        match caps.name("letter") {
            Some(c) => {
                rule_list.insert(rule_no, Letter(c.as_str().chars().next().unwrap()));
            }
            None => {
                let rule: Vec<Vec<u32>> = caps[3]
                    .split(" | ")
                    .map(|x| x.split(" ").map(|y| y.parse::<u32>().unwrap()).collect())
                    .collect();
                rule_list.insert(rule_no, Compound(rule));
            }
        }
    }
    let mut part1 = 0;

    for l in split_input[1].lines() {
        if parse(&rule_list)(l).is_ok() {
            part1 += 1;
        }
    }

    println!("Part 1: {}", part1);

    let mut part2 = 0;

    for l in split_input[1].lines() {
        if let Ok((_, true)) = parse2(&rule_list)(l) {
            part2 += 1;
        }
    }

    println!("Part 2: {}", part2);
}
