use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::iter;
use std::mem;

fn main() {
    let grid: Grid = Grid(
        include_str!("../../input/15.txt")
            .lines()
            .map(|line| line.bytes().map(|b| Entity::parse(b)).collect())
            .collect(),
    );

    println!("{}", p1(grid.clone()));
    println!("{}", p2(&grid));
}

fn p1(mut grid: Grid) -> u32 {
    let (round, hp) = simulate(&mut grid);
    round * hp
}

fn p2(grid: &Grid) -> u32 {
    let initial = grid.units().filter(|unit| unit.race == Race::Elf).count();

    for strength in 4.. {
        let mut g = grid.clone();

        for entity in g.0.iter_mut().flatten() {
            if let Entity::Unit(unit) = entity {
                if unit.race == Race::Elf {
                    unit.strength = strength;
                }
            }
        }

        let (round, hp) = simulate(&mut g);

        if g.units().filter(|unit| unit.race == Race::Elf).count() == initial {
            return round * hp;
        }
    }

    unreachable!()
}

fn simulate(grid: &mut Grid) -> (u32, u32) {
    for round in 0.. {
        if let (complete, Some(hp)) = grid.tick() {
            return (round + complete as u32, hp);
        }
    }

    unreachable!()
}

#[derive(Clone, Debug)]
struct Grid(Vec<Vec<Entity>>);

impl Grid {
    fn tick(&mut self) -> (bool, Option<u32>) {
        let mut moved = HashSet::new();

        for (y, x) in iproduct!(0..self.0.len() as i16, 0..self.0[0].len() as i16) {
            if let (Entity::Unit(unit), false) = (self[(x, y)], moved.contains(&(x, y))) {
                if let Some(hp) = self.victory() {
                    return (false, Some(hp));
                }

                moved.insert(unit.tick(self, (x, y)));
            }
        }

        (true, self.victory())
    }

    fn victory(&self) -> Option<u32> {
        match self
            .units()
            .fold((0u32, 0u32), |(elves, goblins), unit| match unit.race {
                Race::Elf => (elves + unit.hp as u32, goblins),
                Race::Goblin => (elves, goblins + unit.hp as u32),
            }) {
            (elves, 0) => Some(elves),
            (0, goblins) => Some(goblins),
            _ => None,
        }
    }

    fn units(&self) -> impl Iterator<Item = &Unit> {
        self.0.iter().flatten().filter_map(|entity| entity.unit())
    }
}

impl std::ops::Index<Point> for Grid {
    type Output = Entity;

    fn index(&self, point: Point) -> &Self::Output {
        &self.0[point.1 as usize][point.0 as usize]
    }
}

impl std::ops::IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.0[point.1 as usize][point.0 as usize]
    }
}

#[derive(Copy, Clone, Debug)]
enum Entity {
    Wall,
    Space,
    Unit(Unit),
}

impl Entity {
    fn parse(b: u8) -> Self {
        match b {
            b'#' => Self::Wall,
            b'.' => Self::Space,
            b'E' => Self::Unit(Unit::new(Race::Elf, 3)),
            b'G' => Self::Unit(Unit::new(Race::Goblin, 3)),
            _ => unreachable!(),
        }
    }

    fn unit(&self) -> Option<&Unit> {
        match self {
            Entity::Unit(unit) => Some(unit),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Unit {
    race: Race,
    strength: u8,
    hp: u8,
}

impl Unit {
    fn new(race: Race, strength: u8) -> Unit {
        Unit {
            race,
            strength,
            hp: 200,
        }
    }

    fn tick(&self, grid: &mut Grid, start: Point) -> Point {
        if self.attack(grid, start) {
            start
        } else {
            let moved = self.step(grid, start);
            self.attack(grid, moved);
            moved
        }
    }

    fn step(&self, grid: &mut Grid, start: Point) -> Point {
        let mut queue = vec![start];
        let mut queue2 = vec![];
        let mut trails: HashMap<_, _> = HashMap::new();
        let mut enemies = vec![];

        while !queue.is_empty() {
            while let Some(position) = queue.pop() {
                for destination in neighbours(position) {
                    if let Entity::Unit(unit) = grid[destination] {
                        if self.race != unit.race {
                            enemies.push(position);
                        }
                    } else if let Entity::Space = grid[destination] {
                        if !trails.contains_key(&destination) {
                            queue2.push(destination);
                        }

                        trails.entry(destination).or_insert(position);
                    }
                }
            }

            if enemies.is_empty() {
                mem::swap(&mut queue, &mut queue2);
                queue.reverse();
            } else {
                let target = enemies.iter().min_by_key(|(x, y)| (y, x));

                let to = *iter::successors(target, |p| trails.get(p))
                    .filter(|&&point| point != start)
                    .last()
                    .unwrap();

                grid[start] = mem::replace(&mut grid[to], Entity::Unit(*self));

                return to;
            }
        }

        start
    }

    fn attack(&self, grid: &mut Grid, position: Point) -> bool {
        if let Some(target) = neighbours(position)
            .filter(|&p| grid[p].unit().filter(|u| u.race != self.race).is_some())
            .min_by_key(|&p| grid[p].unit().map(|u| u.hp))
        {
            if let Entity::Unit(victim) = &mut grid[target] {
                victim.hp = victim.hp.saturating_sub(self.strength);

                if victim.hp == 0 {
                    grid[target] = Entity::Space;
                }

                return true;
            }
        }

        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Race {
    Elf,
    Goblin,
}

type Point = (i16, i16);

fn neighbours(p: Point) -> impl Iterator<Item = Point> {
    [(0, -1), (-1, 0), (1, 0), (0, 1)]
        .iter()
        .map(move |d| (p.0 + d.0, p.1 + d.1))
}
