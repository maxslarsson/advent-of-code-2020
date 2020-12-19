use std::convert::TryInto;

pub fn part1(input: String) {
    println!("{}", shared(Ship::new(), &input));
}

pub fn part2(input: String) {
    println!("{}", shared(ShipWithWaypoint::new(), &input));
}

fn shared<S>(mut ship: S, input: &str) -> usize
where
    S: Actions,
{
    let input = setup(input);

    for action in input {
        let c = action.chars().next().unwrap();
        let amount: String = action.chars().skip(1).collect();
        let amount: isize = amount.parse().unwrap();
        match c {
            'N' => ship.north_south(amount),
            'S' => ship.north_south(-amount),
            'E' => ship.east_west(amount),
            'W' => ship.east_west(-amount),
            'L' => ship.right_left(-amount),
            'R' => ship.right_left(amount),
            'F' => ship.forward_backward(amount),
            _ => panic!("Shouldn't be any other characters..."),
        }
    }

    ship.manhattan_distance()
}

fn setup(input: &str) -> Vec<&str> {
    return input.split('\n').collect();
}

trait Actions {
    fn new() -> Self;
    fn north_south(&mut self, amount: isize);
    fn east_west(&mut self, amount: isize);
    fn forward_backward(&mut self, amount: isize);
    fn right_left(&mut self, amount: isize);
    fn manhattan_distance(&self) -> usize;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Ship {
    x: isize,
    y: isize,
    facing: Direction,
}

impl Actions for Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            facing: Direction::East,
        }
    }

    fn north_south(&mut self, amount: isize) {
        self.y += amount
    }

    fn east_west(&mut self, amount: isize) {
        self.x += amount
    }

    fn forward_backward(&mut self, amount: isize) {
        match self.facing {
            Direction::North => self.north_south(amount),
            Direction::East => self.east_west(amount),
            Direction::South => self.north_south(-amount),
            Direction::West => self.east_west(-amount),
        }
    }

    fn right_left(&mut self, amount: isize) {
        let mut directions = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        if amount.is_negative() {
            directions.reverse();
        }

        let index = directions
            .iter()
            .position(|&dir| dir == self.facing)
            .unwrap();
        let abs: usize = amount.abs().try_into().unwrap();
        let rotations = abs / 90;

        let new = (index + rotations) % directions.len();

        self.facing = directions[new];
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
}

struct ShipWithWaypoint {
    x: isize,
    y: isize,
    ship: Ship,
}

impl Actions for ShipWithWaypoint {
    fn new() -> Self {
        Self {
            x: 10,
            y: 1,
            ship: Ship::new(),
        }
    }
    fn north_south(&mut self, amount: isize) {
        self.y += amount
    }

    fn east_west(&mut self, amount: isize) {
        self.x += amount
    }

    fn forward_backward(&mut self, amount: isize) {
        for _ in 0..amount {
            self.ship.north_south(self.y);
            self.ship.east_west(self.x);
        }
    }

    fn right_left(&mut self, amount: isize) {
        let abs: usize = amount.abs().try_into().unwrap();
        for _ in 0..abs / 90 {
            let (x, y) = match amount {
                a if a.is_positive() => (self.y, -self.x),
                a if a.is_negative() => (-self.y, self.x),
                _ => panic!("Shouldn't be a number that isn't positive nor negative..."),
            };

            self.x = x;
            self.y = y;
        }
    }

    fn manhattan_distance(&self) -> usize {
        self.ship.manhattan_distance()
    }
}
