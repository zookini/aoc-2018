fn main() {
    println!("{}", play(405, 70953));
    println!("{}", play(405, 70953 * 100));
}

fn play(players: usize, marbles: usize) -> usize {
    let mut scores = vec![0; players];
    let mut circle: std::collections::VecDeque<_> = (0..1).collect();

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            circle.rotate_right(8);
            scores[marble % players] += marble + circle.pop_front().unwrap();
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    *scores.iter().max().unwrap()
}
