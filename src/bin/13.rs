use itertools::iproduct;

fn main() {
    let input = include_str!("../../input/13.txt");
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let carts = Cart::parse(&grid);

    println!("Collision at {:?}", p1(&grid, carts.clone()));
    println!("Last Cart at {:?}", p2(&grid, carts));
}

fn p1(grid: &[&[u8]], mut carts: Vec<Cart>) -> (i16, i16) {
    loop {
        tick(grid, &mut carts);

        if let Some(cart) = carts.iter().find(|cart| cart.collided) {
            return cart.position;
        }
    }
}

fn p2(grid: &[&[u8]], mut carts: Vec<Cart>) -> (i16, i16) {
    while carts.len() > 1 {
        tick(grid, &mut carts);
        carts.retain(|cart| !cart.collided);
    }

    carts[0].position
}

fn tick(grid: &[&[u8]], carts: &mut Vec<Cart>) {
    carts.sort_by_key(|c| (c.position.1, c.position.0));

    for i in 0..carts.len() {
        carts[i].step(&grid);

        for j in 0..carts.len() {
            if i != j && carts[i].position == carts[j].position {
                carts[i].collided = true;
                carts[j].collided = true;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Cart {
    position: (i16, i16),
    direction: u8,
    turn: i16,
    collided: bool,
}

impl Cart {
    fn parse(grid: &[&[u8]]) -> Vec<Cart> {
        iproduct!(0..grid[0].len(), 0..grid.len())
            .filter_map(|(x, y)| Cart::new((x as i16, y as i16), grid[y][x]))
            .collect()
    }

    fn new(position: (i16, i16), direction: u8) -> Option<Cart> {
        Some(Cart {
            position,
            direction: b"^>v<".iter().position(|&b| b == direction)? as u8,
            turn: -1,
            collided: false,
        })
    }

    fn step(&mut self, grid: &[&[u8]]) {
        self.position.0 += [0, 1, 0, -1][self.direction as usize];
        self.position.1 += [-1, 0, 1, 0][self.direction as usize];

        self.direction = match grid[self.position.1 as usize][self.position.0 as usize] {
            b'\\' => [3, 2, 1, 0][self.direction as usize],
            b'/' => [1, 0, 3, 2][self.direction as usize],
            b'+' => (self.direction as i16 + self.turn).rem_euclid(4) as u8,
            _ => self.direction,
        };

        if grid[self.position.1 as usize][self.position.0 as usize] == b'+' {
            self.turn = ((self.turn + 2) % 3) - 1;
        }
    }
}
