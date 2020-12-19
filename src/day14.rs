use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: String) {
    // let mut memory = vec![0; 2usize.pow(36)];
    // let mut memory = Vec::with_capacity(2usize.pow(36));
    let mut memory = HashMap::new();
    let re = Regex::new(r"mask = [X01]{36}(\nmem\[\d+\] = \d+)+").unwrap();

    for mat in re.find_iter(&input) {
        let lines: Vec<_> = mat.as_str().split('\n').collect();
        let bitmask = &lines[0][7..];

        for &mem in &lines[1..] {
            let end = mem.find('=').unwrap();
            let loc: usize = mem[4..end - 2].parse().unwrap();
            let decimal: usize = mem[end + 2..].parse().unwrap();

            let value = format!("{:036b}", decimal);
            let result: String = value
                .chars()
                .zip(bitmask.chars())
                .map(|c| if c.1 == 'X' { c.0 } else { c.1 })
                .collect();

            let decimal_result = usize::from_str_radix(&result, 2).unwrap();

            memory.insert(loc, decimal_result);
        }
    }

    let sum: usize = memory.values().sum();
    println!("{}", sum);
}

pub fn part2(input: String) {
    // let mut memory = vec![0; 2usize.pow(36)];
    // let mut memory = Vec::with_capacity(2usize.pow(36));
    let mut memory = HashMap::new();
    let re = Regex::new(r"mask = [X01]{36}(\nmem\[\d+\] = \d+)+").unwrap();

    for mat in re.find_iter(&input) {
        let lines: Vec<_> = mat.as_str().split('\n').collect();
        let bitmask = &lines[0][7..];

        for &mem in &lines[1..] {
            let end = mem.find('=').unwrap();
            let loc: usize = mem[4..end - 2].parse().unwrap();
            let decimal: usize = mem[end + 2..].parse().unwrap();

            let address = format!("{:036b}", loc);
            let result: String = address
                .chars()
                .zip(bitmask.chars())
                .map(|c| if c.1 == '0' { c.0 } else { c.1 })
                .collect();

            let indexes_of_x: Vec<usize> = result
                .chars()
                .enumerate()
                .filter(|c| c.1 == 'X')
                .map(|c| c.0)
                .collect();

            let addresses: Vec<String> = vec!["01".chars(); indexes_of_x.len()]
                .into_iter()
                .multi_cartesian_product()
                .map(|x| {
                    result
                        .chars()
                        .enumerate()
                        .map(|c| {
                            if let Some(i) = indexes_of_x.iter().position(|&cc| cc == c.0) {
                                x[i]
                            } else {
                                c.1
                            }
                        })
                        .collect()
                })
                .collect();

            for address in addresses.iter() {
                let address = usize::from_str_radix(address, 2).unwrap();

                memory.insert(address, decimal);
            }
        }
    }

    let sum: usize = memory.values().sum();
    println!("{}", sum);
}
