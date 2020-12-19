use std::collections::HashMap;

pub fn part1(input: String) {
    shared(&input, 2020);
}

pub fn part2(input: String) {
    shared(&input, 30000000);
}

fn shared(input: &str, end_number: usize) {
    let mut index = 1;
    let mut last = 0;
    let mut numbers = HashMap::new();

    {
        let mut iter = input.split('\n').next().unwrap().split(',').peekable();
        while let Some(num) = iter.next() {
            // If the peeked element is NOT some, it is the last one in the iterator, meaning that we do not want to add it to the HashMap as it would then be found later on with the .get(last) call
            if iter.peek().is_some() {
                numbers.insert(num.parse().unwrap(), index);
            } else {
                last = num.parse().unwrap();
            }

            index += 1;
        }
    }

    while index <= end_number {
        let push = match numbers.get(&last) {
            None => 0,
            Some(loc) => index - 1 - loc,
        };

        numbers.insert(last, index - 1);
        last = push;
        index += 1;
    }

    // `nth_number - 1` because the Vec starts at 0, and the "numbers" from the problem start with 0
    println!("{}", last);
}
