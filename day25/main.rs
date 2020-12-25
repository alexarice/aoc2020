fn main() {
    let card_pk: u64 = 1614360;
    let door_pk: u64 = 7734663;

    let rem_value = 20201227;

    let mut value = 1;
    let mut card_loop_size = 0;

    while value != card_pk {
        card_loop_size += 1;
        value = (value * 7) % rem_value;
    }
    println!("{}", card_loop_size);

    let mut value = 1;
    for _ in 0..card_loop_size {
        value = (value * door_pk) % rem_value;
    }

    println!("Part 1: {}", value);
}
