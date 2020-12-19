use std::convert::TryInto;
use std::fmt;

pub fn part1(input: String) {
    let mut layout = Layout::new(&input);

    while !layout.is_repeating() {
        layout.tick_part_1();
    }

    println!("{}", layout.count_occupied_seats());
}

pub fn part2(input: String) {
    let mut layout = Layout::new(&input);

    while !layout.is_repeating() {
        layout.tick_part_2();
    }

    println!("{}", layout.count_occupied_seats());
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

pub struct Layout {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
    repeating: bool,
}

impl Layout {
    pub fn new(input: &str) -> Self {
        let height = input.split("\n").count();
        let width = input.split("\n").next().unwrap().len();
        let seats = input
            .replace("\n", "")
            .chars()
            .map(|c| match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                _ => panic!("Shouldn't be any other characters..."),
            })
            .collect();

        Self {
            width,
            height,
            seats,
            repeating: false,
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn tick_part_1(&mut self) {
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.seats[idx];
                let live_neighbors = self.occupied_neighbor_count_part_1(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: If a seat is empty (L) and there are no occupied seats adjacent to it,
                    // the seat becomes occupied.
                    (Seat::Empty, 0) => Seat::Occupied,
                    // Rule 2: If a seat is occupied (#) and four or more seats adjacent to it
                    // are also occupied, the seat becomes empty.
                    (Seat::Occupied, x) if x >= 4 => Seat::Empty,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.repeating = self.seats == next;

        self.seats = next;
    }

    fn occupied_neighbor_count_part_1(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        let iter_row = [self.height - 1, 0, 1];
        let iter_column = [self.width - 1, 0, 1];

        for &delta_row in &iter_row {
            for &delta_col in &iter_column {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                // This means it wrapped
                if neighbor_row.max(row) - neighbor_row.min(row) > 3 {
                    continue;
                }
                // This means it wrapped
                if neighbor_col.max(column) - neighbor_col.min(column) > 3 {
                    continue;
                }

                let idx = self.get_index(neighbor_row, neighbor_col);

                count += match self.seats[idx] {
                    Seat::Floor => 0,
                    Seat::Empty => 0,
                    Seat::Occupied => 1,
                }
            }
        }

        return count;
    }

    pub fn tick_part_2(&mut self) {
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.seats[idx];
                let live_neighbors = self.occupied_neighbor_count_part_2(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: If a seat is empty (L) and there are no occupied seats adjacent to it,
                    // the seat becomes occupied.
                    (Seat::Empty, 0) => Seat::Occupied,
                    // Rule 2: If a seat is occupied (#) and four or more seats adjacent to it
                    // are also occupied, the seat becomes empty.
                    (Seat::Occupied, x) if x >= 5 => Seat::Empty,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.repeating = self.seats == next;

        self.seats = next;
    }

    fn occupied_neighbor_count_part_2(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        // 0: columns
        // 1: rows
        let modifications = [
            // Up and Left
            (-1, -1),
            // Up
            (0, -1),
            // Up and Right
            (1, -1),
            // Right
            (1, 0),
            // Down and Right
            (1, 1),
            // Down
            (0, 1),
            // Down and Left
            (-1, 1),
            // Left
            (-1, 0),
        ];

        for &mods in &modifications {
            let mut delta_row = 0;
            let mut delta_col = 0;

            loop {
                // Modify it first to skip (0, 0) (current seat)
                delta_col += mods.0;
                delta_row += mods.1;

                let row: isize = row.try_into().unwrap();
                let column: isize = column.try_into().unwrap();
                let height: isize = self.height.try_into().unwrap();
                let width: isize = self.width.try_into().unwrap();

                let neighbor_row = row + delta_row;
                let neighbor_col = column + delta_col;

                if neighbor_row > height - 1
                    || neighbor_row < 0
                    || neighbor_col > width - 1
                    || neighbor_col < 0
                {
                    break;
                }

                // // This means it wrapped
                // if neighbor_row.max(row) - neighbor_row.min(row) > 3 {
                //     continue;
                // }
                // // This means it wrapped
                // if neighbor_col.max(column) - neighbor_col.min(column) > 3 {
                //     continue;
                // }

                let idx = self.get_index(
                    neighbor_row.try_into().unwrap(),
                    neighbor_col.try_into().unwrap(),
                );

                match self.seats[idx] {
                    Seat::Occupied => {
                        count += 1;
                        break;
                    }
                    Seat::Empty => break,
                    Seat::Floor => (),
                }
            }
        }

        return count;
    }

    pub fn is_repeating(&self) -> bool {
        self.repeating
    }

    pub fn count_occupied_seats(&self) -> usize {
        self.seats.iter().filter(|&n| *n == Seat::Occupied).count()
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.seats.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = match cell {
                    Seat::Floor => '.',
                    Seat::Empty => 'L',
                    Seat::Occupied => '#',
                };

                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
