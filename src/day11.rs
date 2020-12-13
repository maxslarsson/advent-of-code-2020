pub fn part1(input: String) {
    let layout = Layout::new(&input);
}

pub fn part2(input: String) {
    
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seat {
    Floor = 0,
    Empty = 0,
    Occupied = 1,
}

pub struct Layout {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

impl Layout {
    pub fn new(input: &str) -> Self {
        let height = input.split("\n").count();
        let width = input.split("\n").next().unwrap().len();
        let seats = input.replace("\n", "").chars().map(|c| match c {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied
        }).collect();

        Self {
            width,
            height,
            seats
        }
        
        // let cells = (0..width * height)
        //     .map(|i| {
        //         if i % 2 == 0 || i % 7 == 0 {
        //             Cell::Alive
        //         } else {
        //             Cell::Dead
        //         }
        //     })
        //     .collect();

        // Universe {
        //     width,
        //     height,
        //     cells,
        // }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn occupied_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.seats[idx] as u8;
            }
        }
        
        return count;
    }

    pub fn tick(&mut self) {
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.seats[idx];
                let live_neighbors = self.occupied_neighbor_count(row, col);

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

        self.cells = next;
    }
}