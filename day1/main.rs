use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");

    let numbers : Vec<i32> = input.split("\n").filter(|x| x.len() > 0).map(|x| x.parse().expect("Parsing failed")).collect();

    let len = numbers.len();

    for i in 0 .. len {
	for j in i + 1 .. len {
	    let (ni,nj) = (numbers[i],numbers[j]);
	    if ni + nj == 2020 {
		println!("{} * {} = {}",ni,nj,ni*nj);
	    }
	}
    }

    for i in 0 .. len {
	for j in i + 1 .. len {
	    for k in j + 1 .. len {
		let (ni, nj, nk) = (numbers[i],numbers[j],numbers[k]);
		if ni + nj + nk == 2020 {
		    println!("{} * {} * {} = {}",ni,nj,nk,ni*nj*nk);
		}
	    }
	}
    }
}
