use std::collections::HashMap;

fn setup(input: String) -> Vec<String> {
    return input.split("\n").map(|rule| rule.to_string()).collect()
}

pub fn part1(input: String) {
    let input = setup(input);

    let mut map = HashMap::new();

    for rule in input {
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

            contained_bags.insert(coll[1..3].join(" "), coll[0].to_string());
        }

        map.insert(bag.join(" "), contained_bags);
    }
}

pub fn part2(input: String) {
    
}