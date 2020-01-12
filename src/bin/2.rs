use itertools::{izip, Itertools};

fn main() {
    let ids: Vec<_> = include_str!("../../input/2.txt").lines().collect();

    println!("{}", p1(&ids));
    println!("{}", p2(&ids));
}

fn p1(ids: &[&str]) -> usize {
    let (pairs, triples) = ids
        .iter()
        .map(|id| counts(id.as_bytes()))
        .fold((0, 0), |acc, t| (acc.0 + t.0, acc.1 + t.1));

    pairs * triples
}

fn counts(letters: &[u8]) -> (usize, usize) {
    let mut histogram = [0; 26];

    for letter in letters {
        histogram[(letter - b'a') as usize] += 1;
    }

    (histogram.contains(&2).into(), histogram.contains(&3).into())
}

fn p2(ids: &[&str]) -> String {
    ids.iter()
        .tuple_combinations()
        .find(|(s1, s2)| same(s1, s2).count() == s1.len() - 1)
        .map(|(s1, s2)| same(s1, s2).collect())
        .unwrap()
}

fn same<'a>(s1: &'a str, s2: &'a str) -> impl Iterator<Item = char> + 'a {
    izip!(s1.chars(), s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
}
