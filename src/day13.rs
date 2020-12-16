pub fn part1(input: String) {
    let split: Vec<&str> = input.split("\n").collect();
    let earliest: usize = split[0].parse().unwrap();
    let buses: Vec<usize> = split[1]
        .split(",")
        .filter_map(|bus| bus.parse().ok())
        .collect();

    let mut timestamp = earliest;

    loop {
        let mut found = false;
        for bus_id in &buses {
            if timestamp % bus_id == 0 {
                println!("{}", bus_id * (timestamp - earliest));
                found = true;
                break;
            }
        }

        if found {
            break;
        }

        timestamp += 1;
    }
}

// Extremely slow: Took 269m 26s 620ms 217µs
pub fn part2(input: String) {
    let split: Vec<&str> = input.split("\n").collect();
    let earliest: usize = split[0].parse().unwrap();

    let mut iter: Vec<(usize, usize)> = split[1]
        .split(",")
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();

    // Sort to get largest number first at index 0
    iter.sort_unstable_by_key(|x| x.1);
    iter.reverse();

    let biggest = iter[0];
    let mut timestamp = earliest;
    // let mut timestamp: usize = 100_000_000_000_000;

    while (timestamp + biggest.0) % biggest.1 != 0 {
        timestamp += 1;
    }

    let len = iter.len();

    loop {
        // Print timestamp every 50,000,000 numbers checked
        if (timestamp + biggest.0) % (50_000_000 * biggest.1) == 0 {
            println!("{}", timestamp);
        }

        // We know the first element is a multiple of the timestamp
        let found = iter[1..]
            .iter()
            .map(|(i, bus_id)| ((timestamp + i) % bus_id == 0).then_some(()))
            .take_while(|x| x.is_some())
            .count();

        // Minus 1 because iterator skips first element
        if found == len - 1 {
            break;
        }

        timestamp += biggest.1;
    }

    println!("{}", timestamp);
}
