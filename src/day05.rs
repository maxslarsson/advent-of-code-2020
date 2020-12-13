use std::cmp;

fn setup(input: &str) -> Vec<&str> {
    return input.split("\n").collect();
}

fn calculate_seat_ids(input: Vec<&str>) -> Vec<usize> {
    let mut calculated_rows = Vec::new();
    let mut calculated_columns = Vec::new();

    for line in input {
        let mut rows: Vec<usize> = (0..128).collect();
        let mut columns: Vec<usize> = (0..8).collect();

        for c in line.chars() {
            if c == 'F' {
                rows.drain(rows.len() / 2..);
            } else if c == 'B' {
                rows.drain(..rows.len() / 2);
            } else if c == 'R' {
                columns.drain(..columns.len() / 2);
            } else if c == 'L' {
                columns.drain(columns.len() / 2..);
            }
        }

        assert_eq!(rows.len(), 1);
        assert_eq!(columns.len(), 1);

        calculated_rows.push(rows[0]);
        calculated_columns.push(columns[0]);
    }

    return calculated_rows
        .into_iter()
        .zip(calculated_columns)
        .map(|(row, column)| row * 8 + column)
        .collect();
}

pub fn part1(input: String) {
    let input = setup(&input);

    let seat_ids = calculate_seat_ids(input);

    println!("{}", seat_ids.into_iter().max().unwrap());
}

pub fn part2(input: String) {
    let input = setup(&input);

    let mut seat_ids = calculate_seat_ids(input);

    seat_ids.sort();
    seat_ids.dedup();

    let mut possible = Vec::new();

    for a in &seat_ids {
        for b in &seat_ids {
            if cmp::max(a, b) - cmp::min(a, b) == 2 {
                let between = cmp::max(a, b) - 1;
                if !seat_ids.contains(&between) {
                    possible.push(between);
                }
            }
        }
    }

    possible.sort();
    possible.dedup();

    println!("{}", possible[0]);
}
