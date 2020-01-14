use itertools::{iproduct, Itertools};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const EXTENT: std::ops::Range<i16> = 0..512;

fn main() {
    let positions: Vec<Point> = include_str!("../../input/6.txt")
        .lines()
        .map(|line| {
            line.split(", ")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    println!("{:?}", p1(&positions));
    println!("{:?}", p2(&positions));
}

fn p1(ps: &[Point]) -> usize {
    let grid: Vec<Vec<_>> = EXTENT
        .map(|y| EXTENT.map(|x| nearest((x, y), ps)).collect())
        .collect();

    let infinites: HashSet<_> = grid[0]
        .iter()
        .chain(grid[grid.len() - 1].iter())
        .chain(grid.iter().map(|row| &row[0]))
        .chain(grid.iter().map(|row| &row[row.len() - 1]))
        .flatten()
        .collect();

    let mut histogram = HashMap::new();

    for nearest in grid.iter().flatten().flatten() {
        if !infinites.contains(nearest) {
            *histogram.entry(nearest).or_insert(0) += 1;
        }
    }

    *histogram.values().max().unwrap()
}

type Point = (i16, i16);

fn nearest(p: Point, positions: &[Point]) -> Option<Point> {
    match positions
        .iter()
        .fold(((0, 0), std::usize::MAX, false), |nearest, &position| {
            let distance = distance(p, position);

            match distance.cmp(&nearest.1) {
                Ordering::Less => (position, distance, false),
                Ordering::Equal => (position, distance, true),
                Ordering::Greater => nearest,
            }
        }) {
        (position, _, false) => Some(position),
        (_, _, true) => None,
    }
}

fn distance(a: Point, b: Point) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn p2(ps: &[Point]) -> usize {
    iproduct!(EXTENT, EXTENT)
        .map(|(x, y)| ps.iter().map(|&p| distance((x, y), p)).sum::<usize>())
        .filter(|&distance| distance < 10000)
        .count()
}
