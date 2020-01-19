fn main() {
    let changes: Vec<isize> = include_str!("../../input/1.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", p1(&changes));
    println!("{}", p2(&changes));
}

fn p1(changes: &[isize]) -> isize {
    changes.iter().sum()
}

fn p2(changes: &[isize]) -> isize {
    let mut frequency = 0;
    let mut seen = std::collections::HashSet::new();

    for change in changes.iter().cycle() {
        seen.insert(frequency);
        frequency += change;

        if seen.contains(&frequency) {
            break;
        }
    }

    frequency
}
