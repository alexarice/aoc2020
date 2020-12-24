use nom::{character::complete::one_of, multi::many1, IResult};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Add;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct HexCoord {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for HexCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

enum Direction {
    W,
    NW,
    NE,
    E,
    SE,
    SW,
}
use Direction::*;

impl HexCoord {
    fn from_direction(d: &Direction) -> Self {
        let mut c = HexCoord { x: 0, y: 0, z: 0 };
        match d {
            W => {
                c.x -= 1;
                c.y += 1;
            }
            NW => {
                c.y += 1;
                c.z -= 1;
            }
            NE => {
                c.x += 1;
                c.z -= 1;
            }
            E => {
                c.x += 1;
                c.y -= 1;
            }
            SE => {
                c.y -= 1;
                c.z += 1;
            }
            SW => {
                c.x -= 1;
                c.z += 1;
            }
        }
        c
    }
}

fn parse_dir(input: &str) -> IResult<&str, Direction> {
    let (input, c) = one_of("nsew")(input)?;
    match c {
        'n' => {
            let (input, d) = one_of("ew")(input)?;
            match d {
                'e' => Ok((input, NE)),
                'w' => Ok((input, NW)),
                _ => panic!("Can't get here"),
            }
        }
        's' => {
            let (input, d) = one_of("ew")(input)?;
            match d {
                'e' => Ok((input, SE)),
                'w' => Ok((input, SW)),
                _ => panic!("Can't get here"),
            }
        }
        'e' => Ok((input, E)),
        'w' => Ok((input, W)),
        _ => panic!("Can't get here"),
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_dir)(input)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines = input.lines();

    let plines: Vec<Vec<Direction>> = lines.map(|s| parse_line(s).unwrap().1).collect();

    let movements: Vec<HexCoord> = plines
        .iter()
        .map(|l| {
            l.iter()
                .map(|x| HexCoord::from_direction(x))
                .fold(HexCoord { x: 0, y: 0, z: 0 }, |acc, e| acc + e)
        })
        .collect();

    let mut visited: HashSet<HexCoord> = HashSet::new();

    for m in movements {
        if visited.contains(&m) {
            visited.remove(&m);
        } else {
            visited.insert(m);
        }
    }

    let part1 = visited.len();

    println!("Part 1: {}", part1);

    for _ in 1..=100 {
        let mut neighbours: HashMap<HexCoord, usize> = HashMap::new();

        for m in visited.iter() {
            for d in [W, NW, NE, E, SE, SW].iter() {
                let p = HexCoord::from_direction(d) + *m;
                let n = *neighbours.get(&p).unwrap_or(&0);
                neighbours.insert(p, n + 1);
            }
        }
        for m in visited.iter() {
            if !neighbours.contains_key(&m) {
                neighbours.insert(*m, 0);
            }
        }
        for (m, n) in neighbours.into_iter() {
            if visited.contains(&m) && (n == 0 || n > 2) {
                visited.remove(&m);
            } else if (!visited.contains(&m)) && n == 2 {
                visited.insert(m);
            }
        }
    }

    let part2 = visited.len();

    println!("Part 2: {}", part2);
}
