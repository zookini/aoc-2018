use itertools::iproduct;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;
use std::mem;

type Point = (i16, i16);

const DIRECTIONS: &[Point] = &[(0, -1), (-1, 0), (1, 0), (0, 1)];

fn main() {
    let mut grid: Vec<Vec<u8>> = include_str!("../../input/15.txt")
        .lines()
        .map(|line| line.bytes().collect())
        .collect();

    tick(&mut grid);
    tick(&mut grid);
    tick(&mut grid);
}

fn tick(grid: &mut Vec<Vec<u8>>) {
    let mut moved = HashSet::new();

    for (y, x) in iproduct!(0..grid.len(), 0..grid[0].len()) {
        if !moved.contains(&(x as i16, y as i16)) && [b'G', b'E'].contains(&grid[y][x]) {
            moved.insert(Entity::new(grid, (x as i16, y as i16)).tick(grid));
            draw(grid);
        }
    }
}

fn draw(grid: &Vec<Vec<u8>>) {
    for row in grid {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}

#[derive(Debug)]
struct Entity {
    class: u8,
    position: Point,
}

impl Entity {
    fn new(grid: &Vec<Vec<u8>>, position: Point) -> Entity {
        Entity {
            class: grid[position.1 as usize][position.0 as usize],
            position,
        }
    }

    fn tick(&mut self, grid: &mut Vec<Vec<u8>>) -> Point {
        let mut queue: VecDeque<Point> = iter::once(self.position).collect();
        let mut trails: HashMap<Point, Point> = HashMap::new();

        while let Some(position) = queue.pop_front() {
            for dir in DIRECTIONS {
                let target = Entity::new(grid, (position.0 + dir.0, position.1 + dir.1));

                if [b'G', b'E'].contains(&target.class) && target.class != self.class {
                    return iter::successors(Some(&position), |p| trails.get(p))
                        .filter(|&&position| position != self.position)
                        .last()
                        .map(|&go| {
                            grid[self.position.1 as usize][self.position.0 as usize] =
                                mem::replace(&mut grid[go.1 as usize][go.0 as usize], self.class);
                            go
                        })
                        .unwrap_or(position);
                } else if target.class == b'.' && !trails.contains_key(&target.position) {
                    trails.entry(target.position).or_insert(position);
                    queue.push_back(target.position);
                }
            }
        }

        self.position
    }
}
