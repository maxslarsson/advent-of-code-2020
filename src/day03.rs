fn setup(input: String) -> Vec<Vec<char>> {
    return input.split("\n").map(|row| row.chars().collect()).collect();
}

fn helper(map: &Vec<Vec<char>>, slope_right: usize, slope_left: usize) -> usize {
    // Keep track of current location - (x, y)
    let mut location = (0, 0);
    // Keep track of number of trees encountered
    let mut trees = 0;

    // Y position <= number of rows
    loop {
        // Move by slope
        location.0 += slope_right;
        location.1 += slope_left;
        // X wraps around to beginning if past edge
        location.0 %= map[0].len();

        if location.1 >= map.len() {
            break;
        }

        let cur = map[location.1][location.0];

        // Check current position for trees
        if cur == '#' {
            trees += 1;
        }
    }

    return trees;
}

pub fn part1(input: String) {
    let map = setup(input);
    println!("{}", helper(&map, 3, 1));
}

pub fn part2(input: String) {
    let map = setup(input);
    let sum = helper(&map, 1, 1)
        * helper(&map, 1, 2)
        * helper(&map, 3, 1)
        * helper(&map, 5, 1)
        * helper(&map, 7, 1);
    println!("{}", sum);
}
