fn setup(input: String) -> Vec<usize> {
    // Convert input string to a Vec of usize
    return input
        .split("\n")
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect();
}

pub fn part1(input: String) {
    let input = setup(input);
    let mut res = None;

    for (ia, a) in input.iter().enumerate() {
        for (ib, b) in input.iter().enumerate() {
            if ia == ib {
                continue;
            }

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

    for (ia, a) in input.iter().enumerate() {
        for (ib, b) in input.iter().enumerate() {
            for (ic, c) in input.iter().enumerate() {
                if (ia == ib) || (ib == ic) || (ic == ia) {
                    continue;
                }

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
