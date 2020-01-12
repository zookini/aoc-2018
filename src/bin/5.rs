fn main() {
    let polymer = include_bytes!("../../input/5.txt");

    println!("{}", p1(polymer));
    println!("{}", p2(polymer));
}

fn p1(polymer: &[u8]) -> usize {
    react(polymer).len()
}

fn p2(polymer: &[u8]) -> usize {
    (b'a'..b'z')
        .map(|unit| {
            polymer
                .iter()
                .filter(|&&b| b | 0x60 != unit)
                .copied()
                .collect::<Vec<_>>()
        })
        .map(|polymer| react(&polymer).len())
        .min()
        .unwrap()
}

fn react(polymer: &[u8]) -> Vec<u8> {
    polymer.iter().fold(vec![], |mut result, a| {
        if let Some(_) = result.last().filter(|&b| a | 0x60 == b | 0x60 && a != b) {
            result.pop();
        } else {
            result.push(*a);
        }

        result
    })
}
