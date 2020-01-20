fn main() {
    let digits = [8, 9, 0, 6, 9, 1];
    let recipes = join(&digits);

    let mut scores = vec![3, 7];
    let mut elf = (0, 1);

    for i in 0.. {
        let combined = scores[elf.0] + scores[elf.1];

        if combined >= 10 {
            scores.push(combined / 10);
        }

        scores.push(combined % 10);

        elf.0 = (elf.0 + scores[elf.0] as usize + 1) % scores.len();
        elf.1 = (elf.1 + scores[elf.1] as usize + 1) % scores.len();

        if scores[i..].iter().take(digits.len()).eq(&digits) {
            println!("{}", join(&scores[recipes..recipes + 10]));
            println!("{}", i);
            break;
        }
    }
}

fn join(digits: &[u8]) -> usize {
    digits.iter().fold(0, |n, &digit| n * 10 + digit as usize)
}
