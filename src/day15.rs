use std::collections::HashMap;

pub fn part1(input: String) {
    shared(&input, 2020);
}

pub fn part2(input: String) {
    shared(&input, 30000000);
}

fn shared(input: &str, end_number: usize) {
    let mut index = 1;
    let mut numbers = HashMap::new();

    let iter: Vec<&str> = input.split('\n').next().unwrap().split(',').collect();
    //We do not want to add the last element to the HashMap as it would then be found later on with the .get(last) call
    for &num in &iter[..(iter.len() - 1)] {
        numbers.insert(num.parse().unwrap(), index);
        index += 1;
    }

    let mut last = iter[index - 1].parse().unwrap();

    while index <= end_number {
        let push = match numbers.get(&last) {
            None => 0,
            Some(loc) => index - 1 - loc,
        };

        numbers.insert(last, index - 1);
        last = push;
        index += 1;
    }

    // `nth_number - 1` because the Vec starts at 0, and the "numbers" from the problem start with 1
    println!("{}", last);
}
