use aoc::*;
use itertools::izip;
use regex::bytes::Regex;
use std::collections::HashMap;

const MARGIN: usize = 120;

fn main() -> Result<()> {
    let input = include_bytes!("../../input/12.txt");
    let notes = Notes::parse(input)?;

    let state = [
        &[b'.'; MARGIN],
        &Regex::new(r"initial state: ([.#]*)")?
            .captures(input)
            .unwrap()[1],
        &[b'.'; MARGIN],
    ]
    .concat();

    println!("{}", p1(&notes, &state));
    println!("{}", p2(&notes, &state));
    Ok(())
}

struct Notes<'a>(HashMap<&'a [u8], u8>);

impl<'a> Notes<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Notes> {
        Ok(Notes(
            Regex::new(r"([.#]{5}) => (.)")?
                .captures_iter(bytes)
                .map(|cap| (cap.get(1).unwrap().as_bytes(), cap[2][0]))
                .collect(),
        ))
    }

    fn generation(&self, state: &[u8]) -> Vec<u8> {
        [
            &[b'.'; 2],
            &*state
                .windows(5)
                .map(|pattern| *self.0.get(pattern).unwrap_or(&b'.'))
                .collect::<Vec<_>>(),
            &[b'.'; 2],
        ]
        .concat()
    }

    fn sum(&self, state: &[u8]) -> i32 {
        izip!(-(MARGIN as i32).., state)
            .filter(|&(_, &b)| b == b'#')
            .map(|(i, _)| i)
            .sum()
    }
}

fn p1(notes: &Notes, state: &[u8]) -> i32 {
    notes.sum(&(0..20).fold(state.to_vec(), |state, _| notes.generation(&state)))
}

fn p2(notes: &Notes, start: &[u8]) -> u64 {
    let mut state = start.to_vec();

    for i in 0..120 {
        println!("{}, {}", i, notes.sum(&state));
        state = notes.generation(&state);
    }

    // sum converges at 114 = 10600, incrementing by 80 per generation from there
    (50_000_000_000 - 114) * 80 + 10600
}
