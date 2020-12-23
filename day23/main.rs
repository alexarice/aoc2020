use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{FromIterator, IntoIterator};

#[derive(Clone)]
struct Surroundings<T> {
    next: T,
    prev: T,
}

#[derive(Clone)]
struct MyCollection<T> {
    data: HashMap<T, Surroundings<T>>,
    focus: T,
}

impl<T: Eq + Hash + Copy> MyCollection<T> {
    fn insert_after(&mut self, index: T, value: T) {
        let prev = index;
        let next = self.data[&index].next;
        self.data.insert(value, Surroundings { prev, next });
        self.data.insert(
            prev,
            Surroundings {
                prev: self.data[&prev].prev,
                next: value,
            },
        );
        self.data.insert(
            next,
            Surroundings {
                prev: value,
                next: self.data[&next].next,
            },
        );
    }

    fn rotate_left(&mut self) {
        self.focus = self.data[&self.focus].next;
    }
    fn get_front(&self) -> T {
        self.focus
    }
    fn pop_front(&mut self) -> Option<T> {
        let ret = self.focus;
        let Surroundings { prev, next } = self.data[&ret];
        if next == ret {
            None
        } else {
            self.data.insert(
                prev,
                Surroundings {
                    prev: self.data[&prev].prev,
                    next,
                },
            );
            self.data.insert(
                next,
                Surroundings {
                    prev,
                    next: self.data[&next].next,
                },
            );
            self.focus = next;
            Some(ret)
        }
    }

    fn rotate_to(&mut self, to: T) {
        self.focus = to;
    }
}

impl<T: Eq + Hash + Copy> FromIterator<T> for MyCollection<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut c: MyCollection<T>;
        match iter.next() {
            None => panic!("Iterator must be non empty"),
            Some(x) => {
                let mut data = HashMap::new();
                data.insert(x, Surroundings { prev: x, next: x });
                c = MyCollection { data, focus: x };
            }
        }
        for x in iter {
            c.insert_after(c.focus, x);
            c.rotate_left();
        }
        c.rotate_left();
        c
    }
}

struct MCIter<T> {
    c: MyCollection<T>,
    done: bool,
}

impl<T: Eq + Hash + Copy> Iterator for MCIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.c.pop_front().or_else(|| {
                self.done = true;
                Some(self.c.get_front())
            })
        }
    }
}

impl<T: Eq + Hash + Copy> IntoIterator for MyCollection<T> {
    type Item = T;
    type IntoIter = MCIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        MCIter {
            c: self,
            done: false,
        }
    }
}

fn main() {
    let input = [4, 5, 9, 6, 7, 2, 8, 1, 3];

    let mut q: MyCollection<u64> = input.iter().cloned().collect();

    for _ in 0..100 {
        let current = q.get_front();
        q.rotate_left();
        let a = q.pop_front().unwrap();
        let b = q.pop_front().unwrap();
        let c = q.pop_front().unwrap();
        let mut target = current - 1;
        while target == 0 || [a, b, c].iter().find(|&x| *x == target) != None {
            if target == 0 {
                target = 9;
            } else {
                target -= 1;
            }
        }
        q.insert_after(target, a);
        q.insert_after(a, b);
        q.insert_after(b, c);
    }
    q.rotate_to(1);

    let part1 = q
        .into_iter()
        .skip(1)
        .fold("".to_owned(), |acc, s| acc + &s.to_string());

    println!("Part 1: {}", part1);

    let mut q: MyCollection<u64> = input.iter().cloned().chain(10..=1000000).collect();

    for _ in 0..10000000 {
        let current = q.get_front();
        q.rotate_left();
        let a = q.pop_front().unwrap();
        let b = q.pop_front().unwrap();
        let c = q.pop_front().unwrap();
        let mut target = current - 1;
        while target == 0 || [a, b, c].iter().find(|&x| *x == target) != None {
            if target == 0 {
                target = 1000000;
            } else {
                target -= 1;
            }
        }
        q.insert_after(target, a);
        q.insert_after(a, b);
        q.insert_after(b, c);
    }

    q.rotate_to(1);
    q.rotate_left();
    let first = q.get_front();
    q.rotate_left();
    let second = q.get_front();

    let part2 = first * second;

    println!("Part 2: {} * {} = {}", first, second, part2);
}
