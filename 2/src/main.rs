use regex::Regex;

fn main() {
    let contents: Vec<_> = std::include_str!("../input").split("\n").collect();

    let leftright = Regex::new(r"\d+-\d+").unwrap();
    let letter = Regex::new(r"[A-z]:").unwrap();
    let sequence = Regex::new(r": [A-z]*").unwrap();

    let mut valid = 0;

    for line in contents {
        let leftright = leftright.captures(line).unwrap();
        let leftright: Vec<_> = leftright[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let (left, right) = (leftright[0], leftright[1]);

        let letter = letter.captures(line).unwrap();
        let mut letter = letter[0].to_string();
        letter.pop();
        let letter = letter.chars().next().unwrap();

        let sequence = sequence.captures(line).unwrap();
        let mut sequence = sequence[0].to_string();
        sequence.remove(0);
        sequence.remove(0);

        valid += if part2(left, right, letter, sequence) {
            1
        } else {
            0
        };
    }

    println!("{}", valid);
}

fn part1(left: usize, right: usize, letter: char, sequence: String) -> bool {
    let count = sequence.matches(|c| c == letter).count();

    if count >= left && count <= right {
        true
    } else {
        false
    }
}

fn part2(left: usize, right: usize, letter: char, sequence: String) -> bool {
    if (sequence.chars().nth(left - 1).unwrap() == letter)
        ^ (sequence.chars().nth(right - 1).unwrap() == letter)
    {
        true
    } else {
        false
    }
}
