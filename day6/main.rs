use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");

    let groups = || input.split("\n\n");

    let get_yes = |x : &str| {
	let mut s : Vec<_> = x.chars().filter(|c| !c.is_whitespace()).collect();
	s.sort();
	s.dedup();
	s.len()
    };

    let part1 : usize = groups().map(|x| get_yes(x)).sum();

    println!("Part 1: {}", part1);

    let get_yes2 = |x : &str| {
	let people = x.trim().lines().count();
	let mut s : Vec<_> = x.chars().filter(|c| !c.is_whitespace()).collect();
	s.sort();
	let mut last = ' ';
	let mut count = 0;
	let mut yes = 0;

	for x in s {
	    if x == last {
		count += 1;
	    }
	    else {
		last = x;
		if count >= people {
		    yes += 1;
		}
		count = 1;
	    }
	}
	if count >= people {
	    yes += 1;
	}
	yes
    };

    let part2 : usize = groups().map(|x| get_yes2(x)).sum();

    println!("Part 2: {}", part2);
}
