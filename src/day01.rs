fn setup(input: String) -> Vec<usize> {
    return input
        .split("\n")
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
}

pub fn part1(input: String) {
    let input = setup(input);
    let mut res = None;

    for a in input.iter() {
        for b in input.iter() {
            if a + b == 2020 {
                res = Some((*a, *b));
                break;
            }
        }

        if res.is_some() {
            break;
        }
    }

    if let Some(res) = res {
        println!("{}", res.0 * res.1);
    } else {
        println!("Hmm... the two entries were not found!");
    }
}

pub fn part2(input: String) {
    let input = setup(input);
    let mut res = None;

    for a in input.iter() {
        for b in input.iter() {
            for c in input.iter() {
                if a + b + c == 2020 {
                    res = Some((*a, *b, *c));
                    break;
                }
            }

            if res.is_some() {
                break;
            }
        }

        if res.is_some() {
            break;
        }
    }

    if let Some(res) = res {
        println!("{}", res.0 * res.1 * res.2);
    } else {
        println!("Hmm... the three entries were not found!");
    }
}
