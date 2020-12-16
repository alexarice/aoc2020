use regex::Regex;
use std::fs;

struct Field {
    name: String,
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

impl Field {
    fn is_valid(&self, a: usize) -> bool {
        (self.x1 <= a && a <= self.x2) || (self.y1 <= a && a <= self.y2)
    }
}

fn solve(mut to_solve: Vec<(String, Vec<usize>)>) -> Option<Vec<(String, usize)>> {
    if to_solve.len() == 0 {
        Some(Vec::new())
    } else {
        let (name, last_options) = to_solve.pop().unwrap();
        last_options
            .iter()
            .filter_map(|&id: &usize| {
                let copy: Vec<(String, Vec<usize>)> = to_solve
                    .iter()
                    .map(|(n, x)| (n.clone(), x.iter().filter(|&f| f != &id).cloned().collect()))
                    .collect();
                solve(copy).map(|mut v| {
                    v.push((name.clone(), id));
                    v
                })
            })
            .next()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let parts: Vec<_> = input.split("\n\n").collect();

    let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let fields: Vec<Field> = parts[0]
        .lines()
        .map(|s| {
            let caps = re.captures(s).unwrap();
            Field {
                name: caps[1].to_string(),
                x1: caps[2].parse().unwrap(),
                x2: caps[3].parse().unwrap(),
                y1: caps[4].parse().unwrap(),
                y2: caps[5].parse().unwrap(),
            }
        })
        .collect();

    let my_ticket: Vec<usize> = parts[1].lines().collect::<Vec<_>>()[1]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut rest = parts[2].lines();
    rest.next();
    let nearby_tickets: Vec<Vec<usize>> = rest
        .map(|y| y.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut tser = 0;
    let mut valid_tickets: Vec<Vec<usize>> = Vec::new();

    for ticket in nearby_tickets.clone() {
        let mut valid = true;
        for entry in ticket.iter() {
            if fields.iter().all(|f| !f.is_valid(*entry)) {
                tser += entry;
                valid = false;
            }
        }
        if valid {
            valid_tickets.push(ticket)
        };
    }

    println!("Part 1: {}", tser);

    let mut valid_fields: Vec<(String, Vec<usize>)> = Vec::new();

    for (n, f) in fields.iter().enumerate() {
        valid_fields.push((f.name.clone(), Vec::new()));
        for i in 0..valid_tickets[0].len() {
            if (0..valid_tickets.len()).all(|x| f.is_valid(valid_tickets[x][i])) {
                valid_fields[n].1.push(i);
            }
        }
    }

    valid_fields.sort_by_key(|x| x.1.len());
    valid_fields.reverse();

    match solve(valid_fields) {
        None => println!("Oh no"),
        Some(v) => {
            let part2: usize = v
                .iter()
                .filter(|(n, _)| n.starts_with("departure"))
                .map(|(_, x)| my_ticket[*x])
                .product();
            println!("Part 2: {}", part2)
        }
    }
}
