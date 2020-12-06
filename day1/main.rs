use std::fs;

struct MyIter<I> {
    iter : I,
}
impl<I> Iterator for MyIter<I>
where I: Iterator,
      I: Clone,
{
    type Item = (<I as Iterator>::Item,I);

    fn next(&mut self) -> Option<Self::Item> {
	let i = self.iter.next();
	i.and_then(|x| Some((x, self.iter.clone())))
    }
}
impl<I> MyIter<I> {
    fn new(iter : I) -> MyIter<I> {
	MyIter { iter }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");

    let numbers : Vec<i32> = input.split("\n").filter(|x| x.len() > 0).map(|x| x.parse().expect("Parsing failed")).collect();

    let iter = || MyIter::new(numbers.iter());
    for (i , rest) in iter() {
	for j in rest {
	    if i + j == 2020 {
		println!("{} * {} = {}",i,j,i*j);
	    }
	}
    }

    for (i , rest) in iter() {
	for (j , rest) in MyIter::new(rest) {
	    for k in rest {
		if i + j + k == 2020 {
		    println!("{} * {} * {} = {}",i,j,k,i*j*k);
		}
	    }
	}
    }
}
