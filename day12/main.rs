use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

use self::Direction::*;

enum Instruction {
    F(u32),
    M(Direction, u32),
    TL,
    TR,
    TA,
}
use self::Instruction::*;

#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

use std::ops::{Add, Mul};

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, Coord { x, y }: Coord) -> Self::Output {
        Coord {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Mul<Coord> for u32 {
    type Output = Coord;
    fn mul(self, Coord { x, y }: Coord) -> Coord {
        Coord {
            x: self as i32 * x,
            y: self as i32 * y,
        }
    }
}

#[derive(Copy, Clone)]
struct Position {
    dir: Direction,
    c: Coord,
}

fn do_move(c: Coord, d: &Direction, a: &u32) -> Coord {
    let mut nc = c;
    match d {
        N => nc.y -= *a as i32,
        S => nc.y += *a as i32,
        E => nc.x += *a as i32,
        W => nc.x -= *a as i32,
    };
    nc
}

fn perform(i: &Instruction, p: Position) -> Position {
    let mut nc = p;
    match i {
        F(n) => nc.c = do_move(p.c, &(p.dir), n),
        M(dir, n) => nc.c = do_move(p.c, dir, n),
        TL => {
            match nc.dir {
                N => nc.dir = W,
                S => nc.dir = E,
                E => nc.dir = N,
                W => nc.dir = S,
            };
        }
        TR => {
            match nc.dir {
                N => nc.dir = E,
                S => nc.dir = W,
                E => nc.dir = S,
                W => nc.dir = N,
            };
        }
        TA => {
            match nc.dir {
                N => nc.dir = S,
                S => nc.dir = N,
                E => nc.dir = W,
                W => nc.dir = E,
            };
        }
    };
    nc
}

#[derive(Copy, Clone)]
struct Position2 {
    wc: Coord,
    sc: Coord,
}

fn rotate_l(Coord { x, y }: Coord) -> Coord {
    Coord { x: y, y: -x }
}

fn perform2(i: &Instruction, p: Position2) -> Position2 {
    match i {
        F(n) => Position2 {
            wc: p.wc,
            sc: *n * p.wc + p.sc,
        },
        M(dir, n) => Position2 {
            wc: do_move(p.wc, dir, n),
            sc: p.sc,
        },
        TL => Position2 {
            sc: p.sc,
            wc: rotate_l(p.wc),
        },
        TR => Position2 {
            sc: p.sc,
            wc: rotate_l(rotate_l(rotate_l(p.wc))),
        },
        TA => Position2 {
            sc: p.sc,
            wc: rotate_l(rotate_l(p.wc)),
        },
    }
}

fn man_distance(Coord { x, y }: Coord) -> u32 {
    (x.abs() + y.abs()) as u32
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let instructions: Vec<_> = input
        .lines()
        .map(|x| {
            let num = (&x[1..]).parse().unwrap();
            match &x[0..1] {
                "F" => F(num),
                "N" => M(N, num),
                "S" => M(S, num),
                "E" => M(E, num),
                "W" => M(W, num),
                "L" => match num {
                    90 => TL,
                    180 => TA,
                    270 => TR,
                    _ => panic!("not a turn"),
                },
                "R" => match num {
                    90 => TR,
                    180 => TA,
                    270 => TL,
                    _ => panic!("not a turn"),
                },
                _ => panic!("Unrecognised instruction"),
            }
        })
        .collect();

    let part1 = instructions.iter().fold(
        Position {
            dir: E,
            c: Coord { x: 0, y: 0 },
        },
        |p, i| perform(i, p),
    );

    println!("Part 1: {}", man_distance(part1.c));

    let part2 = instructions.iter().fold(
        Position2 {
            wc: Coord { x: 10, y: -1 },
            sc: Coord { x: 0, y: 0 },
        },
        |p, i| perform2(i, p),
    );

    println!("Part 2: {}", man_distance(part2.sc));
}
