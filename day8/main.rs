use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};

use std::collections::HashSet;
use std::fs;
use std::num::ParseIntError;

#[derive(Clone, Copy)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

use Op::*;

#[derive(Clone, Copy)]
struct Command {
    op: Op,
    val: i32,
}

fn op_parser(input: &str) -> IResult<&str, Op> {
    alt((
        value(Nop, tag("nop")),
        value(Acc, tag("acc")),
        value(Jmp, tag("jmp")),
    ))(input)
}

fn num_parser(input: &str) -> IResult<&str, i32> {
    let (input, sign) = alt((value(true, tag("+")), value(false, tag("-"))))(input)?;
    map_res(digit1, move |x: &str| -> Result<i32, ParseIntError> {
        let y: i32 = x.parse::<i32>()?;
        if sign {
            Ok(y)
        } else {
            Ok(-y)
        }
    })(input)
}

fn parser(input: &str) -> IResult<&str, Command> {
    map(
        separated_pair(op_parser, tag(" "), num_parser),
        |(op, val)| Command { op, val },
    )(input)
}

struct Run {
    accum: i32,
    terminated: bool,
}

fn run(instructions: &Vec<Command>) -> Run {
    let mut line: usize = 0;
    let mut accum: i32 = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    let mut terminated = false;

    while !terminated && !visited.contains(&line) {
        if line == instructions.len() {
            terminated = true;
        } else {
            visited.insert(line);
            match instructions[line] {
                Command { op: Nop, val: _ } => {
                    line += 1;
                }
                Command { op: Acc, val } => {
                    accum += val;
                    line += 1;
                }
                Command { op: Jmp, val } => {
                    line = (line as i32 + val) as usize;
                }
            }
        }
    }
    Run { accum, terminated }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = || input.lines();

    let instructions: Vec<_> = lines()
        .map(|x| parser(x).expect("parsing failed").1)
        .collect();

    println!("Part 1: {}", run(&instructions).accum);

    let flip_com = |com| match com {
        Command { op: Nop, val } => Command { op: Jmp, val },
        Command { op: Jmp, val } => Command { op: Nop, val },
        c => c,
    };

    for i in 0..instructions.len() {
        let mut instr2 = instructions.clone();
        instr2[i] = flip_com(instr2[i]);
        let res = run(&instr2);
        if res.terminated {
            println!("Part 2: {}", res.accum)
        }
    }
}
