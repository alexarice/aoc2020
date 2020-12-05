use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");
    let lines = input.lines();


    let get_id = |s : &str| {
	let sb = s.chars().map(|x| if x == 'B' || x == 'R' { '1' } else {'0'}).collect::<String>();
	let front = &sb[..7];
	let back = &sb[7..];
	let row = u32::from_str_radix(front,2).unwrap();
	let column = u32::from_str_radix(back,2).unwrap();
	row * 8 + column
    };

    let mut seats = lines.map(get_id).collect::<Vec<_>>();
    println!("{}", seats.iter().max().unwrap());

    seats.sort();

    for i in 0 .. seats.len()-1 {
	if seats[i+1] - seats[i] == 2 {
	    println!("My seat: {}", seats[i]+1)
	}
    }
}
