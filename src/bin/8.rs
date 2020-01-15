fn main() {
    let input: Vec<_> = include_str!("../../input/8.txt")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", p1(&mut input.iter().copied()));
    println!("{}", p2(&mut input.iter().copied()));
}

fn p1(nums: &mut impl Iterator<Item = usize>) -> usize {
    let (children, metadata) = (nums.next().unwrap(), nums.next().unwrap());
    (0..children).map(|_| p1(nums)).sum::<usize>() + nums.take(metadata).sum::<usize>()
}

fn p2(nums: &mut impl Iterator<Item = usize>) -> usize {
    Node::tree(nums).value()
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn tree(nums: &mut impl Iterator<Item = usize>) -> Node {
        let (children, metadata) = (nums.next().unwrap(), nums.next().unwrap());

        Node {
            children: (0..children).map(|_| Self::tree(nums)).collect(),
            metadata: nums.take(metadata).collect(),
        }
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|&i| self.children.get(i - 1))
                .map(Self::value)
                .sum()
        }
    }
}
