fn setup(input: String) -> Vec<usize> {
    let mut input: Vec<usize> = input
        .split("\n")
        .map(|jolt| jolt.parse().unwrap())
        .collect();

    input.sort_unstable();

    // Add the charging outlet before first adapter
    input.insert(0, 0);

    // The jolt of the device
    input.push(input.last().unwrap() + 3);

    return input;
}

pub fn part1(input: String) {
    let jolts = setup(input);

    let mut current_jolts = 0;

    let mut differences = Vec::new();

    for jolt in jolts {
        differences.push(jolt - current_jolts);
        current_jolts = jolt;
    }

    println!(
        "{}",
        differences.iter().filter(|&n| *n == 1).count()
            * differences.iter().filter(|&n| *n == 3).count()
    );
}

pub fn part2(input: String) {
    let jolts = setup(input);

    let mut total: Vec<Vec<usize>> = Vec::new();
    let mut running = Vec::new();

    for window in jolts.windows(2) {
        match window[1] - window[0] {
            1 => running.push(window[0]),
            2 => panic!("Shouldn't be any adapters 2 jolts apart..."),
            3 => {
                running.push(window[0]);
                total.push(running.clone());
                running.clear();
            }
            _ => panic!("Shouldn't be any adapters further than 3 jolts apart..."),
        }
    }

    let result: usize = total
        .iter()
        .map(|slice| match slice.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("unexpected slice of size N > 5 consecutive 1-diff elements"),
        })
        .product();

    println!("{}", result)
}
