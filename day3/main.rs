use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");

    let lines = || input.lines();

    let height = lines().count();
    let width = lines().next().unwrap().len();

    let raw : Vec<_> = input.bytes().filter(|x| *x == b'.' || *x == b'#').map(|x| x == b'#').collect();



    assert!(raw.len() == height * width);

    let get_pos = |x,y| x + y * width;

    let find_trees = |&right : &usize ,&down : &usize| {
	let (mut x, mut y) = (0,0);
	let mut trees : u32 = 0;

	while y < height {
	    while x >= width {
		x -= width;
	    }
	    if raw[get_pos(x,y)] {
		trees += 1;
	    }
	    x += right;
	    y += down;
	}
	println!("right: {} down: {}, trees: {}\n",right, down, trees);
	return trees;
    };

    let directions : [(usize,usize);5] = [(1,1),(3,1),(5,1),(7,1),(1,2)];

    let product : u32 = directions.iter().map(|(x,y)| find_trees(x,y)).product();
    println!("{}",product);
}
