use itertools::Itertools;

fn main() {
    let (mut positions, velocities): (Vec<(i32, _)>, Vec<_>) =
        regex::Regex::new(r"position=< *(.*?), *(.*?)> velocity=< *(.*?), *(.*?)>")
            .unwrap()
            .captures_iter(include_str!("../../input/10.txt"))
            .map(|cap| (1..=4).map(|i| cap[i].parse().unwrap()).collect::<Vec<_>>())
            .map(|cap| ((cap[0], cap[1]), (cap[2], cap[3])))
            .unzip();

    for i in 0.. {
        if let Some((top, bottom)) = positions.iter().map(|p| p.1).minmax().into_option() {
            if bottom - top <= 9 {
                println!("{} seconds\n", i);
                break;
            }
        }

        for (mut position, velocity) in positions.iter_mut().zip(&velocities) {
            position.0 += velocity.0;
            position.1 += velocity.1;
        }
    }

    let origin = positions.iter().min().unwrap();
    let mut grid = [[b' '; 80]; 10];

    for (x, y) in &positions {
        grid[(y - origin.1) as usize][(x - origin.0) as usize] = b'#';
    }

    for row in grid.iter() {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}
