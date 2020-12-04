use regex::Regex;

fn setup(input: String) -> (Vec<String>, Regex, Regex, Regex) {
    let input = input.split("\n").map(|line| line.to_string()).collect();

    let leftright = Regex::new(r"\d+-\d+").unwrap();
    let letter = Regex::new(r"[A-z]:").unwrap();
    let sequence = Regex::new(r": [A-z]*").unwrap();

    return (input, leftright, letter, sequence);
}

fn parse(
    line: String,
    leftright: &Regex,
    letter: &Regex,
    sequence: &Regex,
) -> (usize, usize, char, String) {
    let leftright = leftright.captures(&line).unwrap();
    let leftright: Vec<_> = leftright[0]
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (left, right) = (leftright[0], leftright[1]);

    let letter = letter.captures(&line).unwrap();
    let mut letter = letter[0].to_string();
    letter.pop();
    let letter = letter.chars().next().unwrap();

    let sequence = sequence.captures(&line).unwrap();
    let mut sequence = sequence[0].to_string();
    sequence.remove(0);
    sequence.remove(0);

    return (left, right, letter, sequence);
}

pub fn part1(input: String) {
    let (contents, leftright, letter, sequence) = setup(input);
    let mut valid = 0;

    for line in contents {
        let (left, right, letter, sequence) = parse(line, &leftright, &letter, &sequence);

        let count = sequence.matches(|c| c == letter).count();

        valid += if count >= left && count <= right {
            1
        } else {
            0
        }
    }

    println!("{}", valid);
}

pub fn part2(input: String) {
    let (contents, leftright, letter, sequence) = setup(input);
    let mut valid = 0;

    for line in contents {
        let (left, right, letter, sequence) = parse(line, &leftright, &letter, &sequence);

        valid += if (sequence.chars().nth(left - 1).unwrap() == letter)
            ^ (sequence.chars().nth(right - 1).unwrap() == letter)
        {
            1
        } else {
            0
        }
    }

    println!("{}", valid);
}
