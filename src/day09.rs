fn setup(input: &str) -> usize {
    let mut elements = Vec::new();
    for line in input.split('\n') {
        let num: usize = line.parse().unwrap();

        if elements.len() < 25 {
            elements.push(num);
            continue;
        }

        let mut found = false;

        for a in &elements {
            for b in &elements {
                if a == b {
                    continue;
                }

                if a + b == num {
                    found = true;
                }
            }

            if found {
                break;
            }
        }

        if found {
            elements.push(num);
            elements.remove(0);
            assert_eq!(elements.len(), 25);
        } else {
            return num;
        }
    }

    panic!("There should be an invalid number!!")
}

pub fn part1(input: String) {
    println!("{}", setup(&input));
}

pub fn part2(input: String) {
    let invalid = setup(&input);

    let elements: Vec<_> = input.split('\n').collect();
    let elements: Vec<usize> = elements
        .into_iter()
        .map(|elem| elem.parse().unwrap())
        .collect();

    for size in 2..elements.len() {
        let mut found = false;

        for elem in elements.windows(size) {
            let sum: usize = elem.iter().sum();
            if sum == invalid {
                found = true;

                println!(
                    "{}",
                    elem.iter().min().unwrap() + elem.iter().max().unwrap()
                );

                break;
            }
        }

        if found {
            break;
        }
    }
}
