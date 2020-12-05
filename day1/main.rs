use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");

    let numbers : Vec<i32> = input.split("\n").filter(|x| x.len() > 0).map(|x| x.parse().expect("Parsing failed")).collect();

    for (index, i) in numbers.iter().enumerate() {
	for j in &numbers[(index + 1)..] {
	    if i + j == 2020 {
		println!("{} * {} = {}",i,j,i*j);
	    }
	}
    }

    for (index1, i) in numbers.iter().enumerate() {
	for (index2, j) in (&numbers[(index1 + 1)..]).iter().enumerate() {
	    for k in &numbers[(index1 + index2 + 2)..] {
		if i + j + k == 2020 {
		    println!("{} * {} * {} = {}",i,j,k,i*j*k);
		}
	    }
	}
    }






}
