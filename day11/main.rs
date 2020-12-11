use std::fs;

#[derive (Clone, Copy,PartialEq)]
enum Seat {
    Floor,
    Occupied,
    Empty
}

use self::Seat::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = || input.lines();

    let height = lines().count();
    let width = lines().next().unwrap().len();

    // let printseat = |x| {
    // 	match x {
    // 	    Occupied => '#',
    // 	    Empty => 'L',
    // 	    Floor => '.'
    // 	}
    // };

    let seats: Vec<_> = input
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
        .collect();

    let get_pos = |x , y| x + y * width;

    let mut seats1 = &mut (seats.clone());
    let mut seats2 = &mut (seats.clone());

    let mut changed = true;

    while changed {
	changed = false;
	for i in 0..height {
	    for j in 0..width {
		let current = seats1[get_pos(j,i)];
		if current == Floor {
		    continue;
		}
		let occupied = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)].iter().map(|(x,y)| {
		    let a = (j as isize) + x;
		    let b = (i as isize) + y;
		    if a >= 0 && (a as usize) < width && b >= 0 && (b as usize) < height {
			Some(seats1[get_pos(a as usize,b as usize)])
		    } else { None }
		}).filter(|&z| z == Some(Occupied)).count();
		if current == Empty && occupied == 0 {
		    seats2[get_pos(j,i)] = Occupied;
		    changed = true;
		}
		else if current == Occupied && occupied >= 4 {
		    seats2[get_pos(j,i)] = Empty;
		    changed = true;
		}
		else {
		    seats2[get_pos(j,i)] = current;
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
		let current = seats1[get_pos(j,i)];
		if current == Floor {
		    continue;
		}
		let occupied = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)].iter().map(|(x,y)| {
		    let mut a = (j as isize) + x;
		    let mut b = (i as isize) + y;
		    while a >= 0 && (a as usize) < width && b >= 0 && (b as usize) < height && seats1[get_pos(a as usize,b as usize)] == Floor {
			a += x;
			b += y;
		    }
		    if a >= 0 && (a as usize) < width && b >= 0 && (b as usize) < height {
			Some(seats1[get_pos(a as usize,b as usize)])
		    } else { None }
		}).filter(|&z| z == Some(Occupied)).count();
		if current == Empty && occupied == 0 {
		    seats2[get_pos(j,i)] = Occupied;
		    changed = true;
		}
		else if current == Occupied && occupied >= 5 {
		    seats2[get_pos(j,i)] = Empty;
		    changed = true;
		}
		else {
		    seats2[get_pos(j,i)] = current;
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
