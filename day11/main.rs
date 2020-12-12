use ndarray::Array;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Seat {
    Floor,
    Occupied,
    Empty,
}

use self::Seat::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = || input.lines();

    let height = lines().count();
    let width = lines().next().unwrap().len();

    let seats = Array::from_shape_vec(
        (height, width),
        input
            .chars()
            .filter_map(|x| {
                if x == '.' {
                    Some(Floor)
                } else if x == 'L' {
                    Some(Empty)
                } else {
                    None
                }
            })
            .collect(),
    )
    .unwrap();

    let add_coord = |[i, j]: [usize; 2], [a, b]: [isize; 2]| {
        let x = (i as isize) + a;
        let y = (j as isize) + b;
        if x >= 0 && (x as usize) < height && y >= 0 && (y as usize) < width {
            Some([x as usize, y as usize])
        } else {
            None
        }
    };

    let mut seats1 = &mut (seats.clone());
    let mut seats2 = &mut (seats.clone());

    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..height {
            for j in 0..width {
                let current = seats1[[i, j]];
                if current == Floor {
                    continue;
                }
                let occupied = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .map(|&(x, y)| add_coord([i, j], [x, y]).map(|z| seats1[z]))
                .filter(|&z| z == Some(Occupied))
                .count();
                if current == Empty && occupied == 0 {
                    seats2[[i, j]] = Occupied;
                    changed = true;
                } else if current == Occupied && occupied >= 4 {
                    seats2[[i, j]] = Empty;
                    changed = true;
                } else {
                    seats2[[i, j]] = current;
                }
            }
        }
        let seats3 = seats2;
        seats2 = seats1;
        seats1 = seats3;
    }

    let part1 = seats1.iter().filter(|&x| x == &Occupied).count();

    println!("Part 1: {}", part1);

    let mut seats1 = &mut (seats.clone());
    let mut seats2 = &mut (seats.clone());

    let mut changed = true;

    while changed {
        changed = false;
        for i in 0..height {
            for j in 0..width {
                let current = seats1[[i, j]];
                if current == Floor {
                    continue;
                }
                let occupied = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .map(|&(x, y)| {
                    let mut c = add_coord([i, j], [x, y]);
                    while c.map(|z| seats1[z]) == Some(Floor) {
                        c = c.and_then(|z| add_coord(z, [x, y]))
                    }
                    c.map(|z| seats1[z])
                })
                .filter(|&z| z == Some(Occupied))
                .count();
                if current == Empty && occupied == 0 {
                    seats2[[i, j]] = Occupied;
                    changed = true;
                } else if current == Occupied && occupied >= 5 {
                    seats2[[i, j]] = Empty;
                    changed = true;
                } else {
                    seats2[[i, j]] = current;
                }
            }
        }
        let seats3 = seats2;
        seats2 = seats1;
        seats1 = seats3;
    }

    let part2 = seats1.iter().filter(|&x| x == &Occupied).count();

    println!("Part 1: {}", part2);
}
