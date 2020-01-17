use itertools::iproduct;
use rayon::prelude::*;

const SERIAL: i32 = 7165;
const MAX: u16 = 300;

fn main() {
    println!("{:?}", p1());
    println!("{:?}", p2());
}

fn p1() -> (u16, u16) {
    best(3).0
}

fn best(size: u16) -> ((u16, u16), i32) {
    iproduct!(1..=MAX - size + 1, 1..=MAX - size + 1)
        .map(|p| (p, powers(p.0, p.1, size)))
        .max_by_key(|&(_, power)| power)
        .unwrap()
}

fn powers(x: u16, y: u16, size: u16) -> i32 {
    iproduct!(x..x + size, y..y + size)
        .map(|(x, y)| power(x, y))
        .sum()
}

fn power(x: u16, y: u16) -> i32 {
    let rack = x as i32 + 10;
    (rack * y as i32 + SERIAL) * rack / 100 % 10 - 5
}

fn p2() -> (u16, u16, u16) {
    let (size, ((x, y), _)) = (1..=MAX)
        .into_par_iter()
        .map(|size| (size, best(size)))
        .max_by_key(|&(_, (_, power))| power)
        .unwrap();

    (x, y, size)
}
