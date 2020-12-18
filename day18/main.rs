use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult,
};
use std::fs;

enum AST {
    Num(u64),
    Add(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
}
use AST::*;

impl AST {
    fn eval(&self) -> u64 {
        match self {
            Num(n) => *n,
            Add(b, c) => b.eval() + c.eval(),
            Mul(b, c) => b.eval() * c.eval(),
        }
    }
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |x: &str| x.parse::<u64>())(input)
}

fn part(input: &str) -> IResult<&str, AST> {
    alt((
        map(number, |x| Num(x)),
        map(tuple((tag("("), ast_parser, tag(")"))), |x| x.1),
    ))(input)
}

fn ast_parser(input: &str) -> IResult<&str, AST> {
    let mut a: AST;
    let mut op_is_plus: Option<bool>;
    let (input, first) = part(input)?;
    a = first;
    let (input, b) = opt(alt((value(true, tag(" + ")), value(false, tag(" * ")))))(input)?;
    op_is_plus = b;
    let mut in_mut = input;
    while let Some(x) = op_is_plus {
        let (input, second) = part(in_mut)?;
        in_mut = input;
        if x {
            a = Add(Box::new(a), Box::new(second));
        } else {
            a = Mul(Box::new(a), Box::new(second));
        }
        let (input, b) = opt(alt((value(true, tag(" + ")), value(false, tag(" * ")))))(in_mut)?;
        op_is_plus = b;
        in_mut = input;
    }
    Ok((in_mut, a))
}

fn part2(input: &str) -> IResult<&str, AST> {
    alt((
        map(number, |x| Num(x)),
        map(tuple((tag("("), ast_parser2, tag(")"))), |x| x.1),
    ))(input)
}

fn add_part(input: &str) -> IResult<&str, AST> {
    let mut a: AST;
    let mut op_is_plus: Option<bool>;
    let (input, first) = part2(input)?;
    a = first;
    let (input, b) = opt(value(true, tag(" + ")))(input)?;
    op_is_plus = b;
    let mut in_mut = input;
    while let Some(_) = op_is_plus {
        let (input, second) = part2(in_mut)?;
        in_mut = input;
        a = Add(Box::new(a), Box::new(second));
        let (input, b) = opt(value(true, tag(" + ")))(in_mut)?;
        op_is_plus = b;
        in_mut = input;
    }
    Ok((in_mut, a))
}

fn ast_parser2(input: &str) -> IResult<&str, AST> {
    let mut a: AST;
    let mut op_is_plus: Option<bool>;
    let (input, first) = add_part(input)?;
    a = first;
    let (input, b) = opt(value(false, tag(" * ")))(input)?;
    op_is_plus = b;
    let mut in_mut = input;
    while let Some(_) = op_is_plus {
        let (input, second) = add_part(in_mut)?;
        in_mut = input;
        a = Mul(Box::new(a), Box::new(second));
        let (input, b) = opt(value(false, tag(" * ")))(in_mut)?;
        op_is_plus = b;
        in_mut = input;
    }
    Ok((in_mut, a))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = || input.lines();

    let part1: u64 = lines().map(|s| ast_parser(s).unwrap().1.eval()).sum();

    println!("Part 1: {}", part1);

    let part2: u64 = lines().map(|s| ast_parser2(s).unwrap().1.eval()).sum();

    println!("Part 2: {}", part2);
}
