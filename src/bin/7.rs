use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("../../input/7.txt");
    let graph = input.lines().map(&pair).into_group_map();

    println!("{}", p1(&graph));
    println!("{}", p2(&graph));
}

fn pair(s: &str) -> (char, char) {
    let bs = s.as_bytes();
    (bs[5] as char, bs[36] as char)
}

fn p1(graph: &HashMap<char, Vec<char>>) -> String {
    let mut result = String::new();
    let mut dependencies = dependencies(graph);
    let mut queue = queue(&dependencies);

    while let Some(Reverse(node)) = queue.pop() {
        result.push(node);

        for child in graph.get(&node).iter().flat_map(|&v| v) {
            match dependencies.get_mut(child) {
                Some(1) | None => queue.push(Reverse(*child)),
                Some(parents) => *parents -= 1,
            }
        }
    }

    result
}

fn dependencies(graph: &HashMap<char, Vec<char>>) -> HashMap<char, u32> {
    let mut dependencies = HashMap::new();

    for &node in graph.keys() {
        dependencies.entry(node).or_default();
    }

    for &node in graph.values().flatten() {
        *dependencies.entry(node).or_default() += 1;
    }

    dependencies
}

fn queue(dependencies: &HashMap<char, u32>) -> BinaryHeap<Reverse<char>> {
    dependencies
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&c, _)| Reverse(c))
        .collect()
}

fn p2(graph: &HashMap<char, Vec<char>>) -> u32 {
    let mut elapsed = 0;
    let mut workers = [Worker::new(None, 0); 5];
    let mut dependencies = dependencies(graph);
    let mut queue = queue(&dependencies);

    loop {
        let time = workers[0].time;

        if time > elapsed {
            elapsed = time;

            if let Some(children) = workers[0].node.take().and_then(|n| graph.get(&n)) {
                for child in children {
                    match dependencies.get_mut(child) {
                        Some(1) | None => queue.push(Reverse(*child)),
                        Some(parents) => *parents -= 1,
                    }
                }
            }
        }

        if let Some(Reverse(node)) = queue.pop() {
            workers[0] = Worker::new(Some(node), elapsed + node as u32 - 4);
            workers.sort_unstable_by_key(|w| w.time);
        } else if let Some(position) = workers.iter().position(|w| w.time > elapsed) {
            workers.rotate_left(position);
        } else {
            return elapsed;
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Worker {
    node: Option<char>,
    time: u32,
}

impl Worker {
    fn new(node: Option<char>, time: u32) -> Worker {
        Worker { node, time }
    }
}
