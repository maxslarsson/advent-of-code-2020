fn main() {
    let contents: Vec<_> = std::include_str!("../input").split("\n").collect();
    let contents: Vec<_> = contents
        .iter()
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();

    let entries: usize = rprompt::prompt_reply_stdout("Number of entries: ")
        .unwrap()
        .parse()
        .unwrap();

    let mut locations: Vec<usize> = vec![0; entries];

    let result = loop {
        // This function increases the "locations" value
        // It is ok to run this before the check for identicals because `locations` is initialized with identical 0s in each element
        // so increasing it will not matter
        for i in (0..locations.len()).rev() {
            if locations[i] == contents.len() - 1 {
                locations[i] = 0;
                continue;
            } else {
                locations[i] += 1;
                break;
            }
        }

        // Check if any of the numbers in locations are the same
        if (1..locations.len()).any(|i| locations[i..].contains(&locations[i - 1])) {
            continue;
        }

        if locations.iter().fold(0, |acc, x| acc + contents[*x]) == 2020 {
            break locations.iter().fold(1, |acc, x| acc * contents[*x]);
        }
    };

    println!("{}", result);
}
