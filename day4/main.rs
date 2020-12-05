extern crate regex;

use regex::Regex;

use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt")
	.expect("Something went wrong reading the file");

    let passports = || input.split("\n\n");

    println!("passports: {}", passports().count());

    let re = Regex::new(r"([a-z]{3}):[^\s]+").unwrap();

    let needed_keys = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
    let mut count = 0;

    for passport in passports() {
	let keys : Vec<_> = re.captures_iter(passport).map(|x| x.get(1).unwrap().as_str()).collect();
	if needed_keys.iter().all(|x| keys.contains(x)) {
	    count += 1;
	}
    }

    println!("{}",count);

    let number_check = move |reg : Regex,low,high| {
	move |x| {
	    if let Some(n) = reg.captures(x).and_then(|s| s[1].parse::<u32>().ok()) {
		n >= low && n <= high
	    } else { false }
	}
    };
    let hasbry = number_check(Regex::new(r"byr:(\d{4})").unwrap(),1920,2002);
    let hasiyr = number_check(Regex::new(r"iyr:(\d{4})").unwrap(),2010,2020);
    let haseyr = number_check(Regex::new(r"eyr:(\d{4})").unwrap(),2020,2030);
    let hashgtcm = number_check(Regex::new(r"hgt:(\d{3})cm").unwrap(),150,193);
    let hashgtin = number_check(Regex::new(r"hgt:(\d{2})in").unwrap(),59,76);
    let hashgt = |x| {hashgtcm(x) || hashgtin(x)};
    let rehcl = Regex::new(r"hcl:#[a-f0-9]{6}").unwrap();
    let hashcl = |x| rehcl.is_match(x);
    let reecl = Regex::new(r"ecl:(?:amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let hasecl = |x| reecl.is_match(x);
    let repid = Regex::new(r"pid:\d{9}").unwrap();
    let haspid = |x| repid.is_match(x);

    let isvalid = |x| {hasbry(x) && hasiyr(x) && haseyr(x) && hashgt(x) && hashcl(x) && hasecl(x) && haspid(x)};

    for x in passports() {
	println!("{},{},{},{},{},{},{},{}",hasbry(x) , hasiyr(x) , haseyr(x) , hashgt(x) , hashcl(x) , hasecl(x) , haspid(x), isvalid(x));
    }

    let valid = || passports().filter(|x| isvalid(x));
    let count2 = valid().count();
    for x in valid() {
	println!("{}\n",x);

    }

    println!("{}",count2);
    // Doesn't actually work. Gives one too many but I have no idea why
}
