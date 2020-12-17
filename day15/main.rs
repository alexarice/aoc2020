use std::collections::HashMap;

fn getnth(num: usize) -> u32 {
    let input = [6, 19, 0, 5, 7, 13, 1];
    let mut memory: HashMap<u32, usize> = HashMap::new();

    for (n, x) in input.iter().enumerate() {
        memory.insert(*x, n + 1);
    }

    let mut turn = input.len();
    let mut next = 0;

    while turn < num - 1 {
        turn += 1;
        let last_used: Option<usize> = memory.get(&next).cloned();
        memory.insert(next, turn);
        match last_used {
            None => {
                next = 0;
            }
            Some(n) => {
                next = turn as u32 - n as u32;
            }
        }
    }
    next
}

fn main() {
    println!("Part 1: {}", getnth(2020));
    println!("Part 2: {}", getnth(30000000));
}
