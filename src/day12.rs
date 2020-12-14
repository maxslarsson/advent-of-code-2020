use std::convert::TryInto;

pub fn part1(input: String) {
    let ship = Ship::new();
    let input = setup(&input);

    for action in input {
        let c = action.chars().nth(0).unwrap();
        let num: usize = action[1..].parse().unwrap();
        match (c, num) {
            ("N", amount) =>
        }
    }
}

pub fn part2(input: String) {}

fn setup(input: &str) -> Vec<&str> {
    return input.split("\n").collect();
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Ship {
    x: isize,
    y: isize,
    facing: Direction
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Direction::East
        }
    }

    fn u_to_i(&self, u: usize) -> isize {
        u.try_into().unwrap()
    }

    pub fn north(&mut self, amount: usize) {
        self.y += self.u_to_i(amount)
    }

    pub fn east(&mut self, amount: usize) {
        self.x += self.u_to_i(amount)
    }

    pub fn west(&mut self, amount: usize) {
        self.x -= self.u_to_i(amount)
    }

    pub fn south(&mut self, amount: usize) {
        self.y -= self.u_to_i(amount)
    }

    pub fn forward(&mut self, amount: usize) {
        match self.facing {
            Direction::North => self.north(amount),
            Direction::East => self.east(amount),
            Direction::South => self.south(amount),
            Direction::West => self.west(amount)
        }
    }

    pub fn left(&mut self, amount: usize) {
        let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

        let index = directions.iter().position(|&dir| dir == self.facing).unwrap();
        let rotations = amount / 90;

        let new = (index - rotations) % directions.len();

        self.facing = directions[new];
    }

    pub fn right(&mut self, amount: usize) {
        let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

        let index = directions.iter().position(|&dir| dir == self.facing).unwrap();
        let rotations = amount / 90;

        let new = (index + rotations) % directions.len();

        self.facing = directions[new];
    }
}