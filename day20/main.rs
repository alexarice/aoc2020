use std::fs;

struct Coord {
    x: usize,
    y: usize,
}

fn rotate(Coord { x, y }: Coord) -> Coord {
    Coord { x: 9 - y, y: x }
}

fn reflect(Coord { x, y }: Coord) -> Coord {
    Coord { x: 9 - x, y }
}

#[derive(Clone, Copy, Debug)]
enum Orient {
    Id,
    R,
    RR,
    RRR,
    F,
    FR,
    FRR,
    FRRR,
}
use Orient::*;

fn getTransform(i: Orient) -> impl Fn(Coord) -> Coord {
    match i {
        Id => |i| i,
        R => |i| rotate(i),
        RR => |i| rotate(rotate(i)),
        RRR => |i| rotate(rotate(rotate(i))),
        F => |i| reflect(i),
        FR => |i| reflect(rotate(i)),
        FRR => |i| reflect(rotate(rotate(i))),
        FRRR => |i| reflect(rotate(rotate(rotate(i)))),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile {
    id: u64,
    size: usize,
    pic: [[bool; 10]; 10],
}

impl Tile {
    fn borderN(&self, o: Orient) -> [bool; 10] {
        let transform = getTransform(o);
        let mut out = [false; 10];
        for i in 0..10 {
            let Coord { x, y } = transform(Coord { x: i, y: 0 });
            out[i] = self.pic[y][x]
        }
        out
    }
    fn borderS(&self, o: Orient) -> [bool; 10] {
        let transform = getTransform(o);
        let mut out = [false; 10];
        for i in 0..10 {
            let Coord { x, y } = transform(Coord { x: i, y: 9 });
            out[i] = self.pic[y][x]
        }
        out
    }
    fn borderW(&self, o: Orient) -> [bool; 10] {
        let transform = getTransform(o);
        let mut out = [false; 10];
        for i in 0..10 {
            let Coord { x, y } = transform(Coord { x: 0, y: i });
            out[i] = self.pic[y][x]
        }
        out
    }
    fn borderE(&self, o: Orient) -> [bool; 10] {
        let transform = getTransform(o);
        let mut out = [false; 10];
        for i in 0..10 {
            let Coord { x, y } = transform(Coord { x: 9, y: i });
            out[i] = self.pic[y][x]
        }
        out
    }
}

fn parse_tile(input: &str) -> Tile {
    let mut lines = input.lines();
    let id = (lines.next().unwrap()[5..9]).parse::<u64>().unwrap();
    let mut pic = [[false; 10]; 10];
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            pic[y][x] = c == '#';
        }
    }
    let size = 10;
    Tile { id, size, pic }
}

fn solve(
    grid_size: usize,
    placed: Vec<(Orient, Tile)>,
    to_place: Vec<Tile>,
) -> Option<Vec<(Orient, Tile)>> {
    let placed_len = placed.len();
    if to_place.len() == 0 {
        Some(placed)
    } else {
        to_place
            .iter()
            .enumerate()
            .map(|(n, &t)| {
                [Id, R, RR, RRR, F, FR, FRR, FRRR]
                    .into_iter()
                    .map(|&o| {
                        let mut can_place = true;
                        if placed_len % grid_size != 0 {
                            let (ol, last) = placed[placed_len - 1];
                            if last.borderE(ol) != t.borderW(o) {
                                can_place = false;
                            }
                        }
                        if placed_len >= grid_size {
                            let (ol, last) = placed[placed_len - grid_size];
                            if last.borderS(ol) != t.borderN(o) {
                                can_place = false;
                            }
                        }
                        if can_place {
                            let mut placed2 = placed.clone();
                            let mut to_place2 = to_place.clone();
                            placed2.push((o, t));
                            to_place2.remove(n);
                            solve(grid_size, placed2, to_place2)
                        } else {
                            None
                        }
                    })
                    .fold(None, |x, y| x.or(y))
            })
            .fold(None, |x, y| x.or(y))
    }
}

#[derive(PartialEq)]
enum Water {
    Space,
    SM,
    NSM,
}
use Water::*;

fn reflect_vec<T>(mut input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for x in input.iter_mut() {
        x.reverse()
    }
    input
}

fn rotate_vec<T: Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let tiles: Vec<_> = input.split("\n\n").map(|x| parse_tile(x)).collect();
    let len = tiles.len();
    let grid_size = match len {
        144 => 12,
        9 => 3,
        _ => 0,
    };

    let solved = solve(grid_size, vec![], tiles).unwrap();

    let part1 = solved[0].1.id
        * solved[grid_size - 1].1.id
        * solved[len - grid_size].1.id
        * solved[len - 1].1.id;

    for x in solved.iter() {
        println!("{},{:?}", x.1.id, x.0);
    }

    for i in (0..grid_size) {
        for k in (0..10) {
            for j in (0..grid_size) {
                for l in (0..10) {
                    let s = solved[i * grid_size + j];
                    let c = getTransform(s.0)(Coord { x: l, y: k });
                    print!("{}", if s.1.pic[c.y][c.x] { '#' } else { '.' })
                }
                print!(" ")
            }
            println!("");
        }
        println!("");
    }

    println!("Part 1: {}", part1);

    let mut big_tile: Vec<Vec<Water>> = vec![];

    for i in (0..grid_size) {
        for k in (1..9) {
            big_tile.push(vec![]);
            for j in (0..grid_size) {
                for l in (1..9) {
                    let s = solved[i * grid_size + j];
                    let c = getTransform(s.0)(Coord { x: l, y: k });
                    big_tile
                        .last_mut()
                        .unwrap()
                        .push(if s.1.pic[c.y][c.x] { NSM } else { Space })
                }
            }
        }
    }

    let sm = fs::read_to_string("sea_monster.txt").expect("Something went wrong reading the file");
    let psm: Vec<Vec<bool>> = sm
        .lines()
        .map(|s| s.chars().map(|x| x == '#').collect())
        .collect();

    let sea_monsters = [
        psm.clone(),
        rotate_vec(psm.clone()),
        rotate_vec(rotate_vec(psm.clone())),
        rotate_vec(rotate_vec(rotate_vec(psm.clone()))),
        reflect_vec(psm.clone()),
        reflect_vec(rotate_vec(psm.clone())),
        reflect_vec(rotate_vec(rotate_vec(psm.clone()))),
        reflect_vec(rotate_vec(rotate_vec(rotate_vec(psm.clone())))),
    ];

    for s in sea_monsters.iter() {
        for x in 0..(big_tile[0].len() - s[0].len() + 1) {
            for y in 0..(big_tile.len() - s.len() + 1) {
                let fits = (0..s[0].len())
                    .all(|i| (0..s.len()).all(|j| big_tile[y + j][x + i] != Space || !s[j][i]));
                if fits {
                    for i in 0..s[0].len() {
                        for j in 0..s.len() {
                            if s[j][i] {
                                big_tile[y + j][x + i] = SM;
                            }
                        }
                    }
                }
            }
        }
    }

    for line in big_tile.iter() {
        for x in line {
            print!(
                "{}",
                match x {
                    Space => ".",
                    NSM => "#",
                    SM => "@",
                }
            );
        }
        println!("")
    }

    let part2: usize = big_tile
        .iter()
        .map(|x| x.iter().filter(|&y| *y == NSM).count())
        .sum();

    println!("Part 2: {}", part2)
}
