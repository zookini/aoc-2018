use aoc::*;

fn main() -> Result<()> {
    let claims = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")?
        .captures_iter(include_str!("../../input/3.txt"))
        .map(|cap| {
            Ok(Claim {
                id: cap[1].parse()?,
                x: cap[2].parse()?,
                y: cap[3].parse()?,
                width: cap[4].parse()?,
                height: cap[5].parse()?,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    println!("{}", p1(&claims));
    println!("{}", p2(&claims));
    Ok(())
}

fn p1(claims: &[Claim]) -> usize {
    cut(claims).iter().flatten().filter(|&&b| b >= 2).count()
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn cut(claims: &[Claim]) -> Vec<Vec<usize>> {
    let mut fabric = vec![vec![0; 1024]; 1024];

    for claim in claims {
        for row in &mut fabric[claim.y..claim.y + claim.height] {
            for overlap in &mut row[claim.x..claim.x + claim.width] {
                *overlap += 1;
            }
        }
    }

    fabric
}

fn p2(claims: &[Claim]) -> usize {
    let fabric = cut(claims);

    claims
        .iter()
        .find(|claim| {
            fabric[claim.y..claim.y + claim.height]
                .iter()
                .all(|row| row[claim.x..claim.x + claim.width].iter().all(|&c| c == 1))
        })
        .unwrap()
        .id
}
