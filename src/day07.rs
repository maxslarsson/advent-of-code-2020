use std::collections::HashMap;

fn setup(input: String) -> HashMap<String, HashMap<String, usize>> {
    let mut map = HashMap::new();
    for rule in input.split("\n") {
        let mut words = rule.split(" ").peekable();
        let bag: Vec<_> = words.by_ref().take(2).collect();

        assert_eq!(bag.len(), 2);

        // Get rid of word "bags"
        words.next();

        let mut contained_bags = HashMap::new();

        loop {
            let coll: Vec<_> = words.by_ref().skip(1).take(3).collect();

            if coll.len() != 3 {
                break;
            }

            if coll == ["no", "other", "bags."] {
                break;
            }

            contained_bags.insert(coll[1..3].join(" "), coll[0].parse().unwrap());
        }

        map.insert(bag.join(" "), contained_bags);
    }

    return map;
}

pub fn part1(input: String) {
    let map = setup(input);

    let mut running = 0;

    for key in map.keys() {
        if key == "shiny gold" {
            continue;
        }

        running += if recursion_part_1(&map, key) { 1 } else { 0 }
    }

    println!("{}", running);
}

fn recursion_part_1(map: &HashMap<String, HashMap<String, usize>>, current: &str) -> bool {
    if current == "shiny gold" {
        return true;
    }

    for key in map.get(current).unwrap().keys() {
        if recursion_part_1(map, key) {
            return true;
        }
    }

    return false;
}

pub fn part2(input: String) {
    let map = setup(input);

    // Subtract 1 because the bag DOES NOT contain itself
    println!("{}", recursion_part_2(&map, "shiny gold") - 1);
}

fn recursion_part_2(map: &HashMap<String, HashMap<String, usize>>, current: &str) -> usize {
    let iter = map.get(current).unwrap().keys().peekable();

    let mut running = 1;
    let local_map = map.get(current).unwrap();

    for key in iter {
        running += local_map.get(key).unwrap() * recursion_part_2(map, key);
    }

    return running;
}
