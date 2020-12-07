use std::collections::{HashMap, HashSet};

fn setup(input: String) -> Vec<String> {
    return input.split("\n\n").map(|group| group.to_string()).collect();
}

pub fn part1(input: String) {
    let input = setup(input);

    let mut groups = Vec::new();

    for group in input {
        let mut yes = HashSet::new();

        for person in group.split("\n") {
            for c in person.chars() {
                yes.insert(c);
            }
        }

        groups.push(yes);
    }

    println!("{}", groups.into_iter().map(|set| set.len()).sum::<usize>());
}

pub fn part2(input: String) {
    let input = setup(input);

    let mut groups = Vec::new();

    for group in input {
        let len = group.split("\n").count();

        let mut yes = HashMap::new();

        for person in group.split("\n") {
            for c in person.chars() {
                // Default is 0 because 1 gets automatically added when it is inserted into the HashMap
                let value = yes.get(&c).unwrap_or(&0) + 1;
                yes.insert(c, value);
            }
        }

        let adequate: Vec<char> = yes
            .into_iter()
            .filter_map(|(key, val)| if val == len { Some(key) } else { None })
            .collect();

        groups.push(adequate);
    }

    println!("{}", groups.into_iter().map(|set| set.len()).sum::<usize>());
}
