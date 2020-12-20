use std::collections::HashMap;

pub fn part1(input: String) {
    let (fields, _your_ticket, nearby_tickets) = setup(&input);

    let invalid_tickets = find_invalid_tickets(&nearby_tickets, &fields);

    let sum: usize = invalid_tickets.iter().map(|ticket| ticket.1).sum();
    println!("{}", sum);
}

pub fn part2(input: String) {
    let (fields, your_ticket, mut nearby_tickets) = setup(&input);

    let invalid_tickets = find_invalid_tickets(&nearby_tickets, &fields);

    let original_length = nearby_tickets.len();

    for inv in invalid_tickets {
        nearby_tickets.remove(inv.0 - (original_length - nearby_tickets.len()));
    }

    nearby_tickets.insert(0, your_ticket.clone());

    // Make sure that each ticket is same length;
    assert_eq!(
        nearby_tickets
            .iter()
            .skip(1)
            .filter(|ticket| ticket.len() == nearby_tickets[0].len())
            .count(),
        // -1 because we skip the first element
        nearby_tickets.len() - 1
    );

    assert_eq!(fields.len(), nearby_tickets[0].len());

    let mut matches = HashMap::new();

    for field in fields {
        for index in 0..nearby_tickets[0].len() {
            let mut count = 0;

            for ticket in &nearby_tickets {
                let elem = ticket[index];
                let field = field.1;

                if (elem >= field.0 .0 && elem <= field.0 .1)
                    || (elem >= field.1 .0 && elem <= field.1 .1)
                {
                    count += 1;
                } else {
                    break;
                }
            }

            if count == nearby_tickets.len() {
                matches.insert(field.0, index);
                break;
            }
        }
    }

    let product: usize = matches
        .into_iter()
        .inspect(|index| println!("{:?}", index))
        .filter(|field| field.0.starts_with("departure"))
        .map(|field| field.1)
        .map(|index| your_ticket[index])
        .product();

    println!("{}", product);
}

fn setup(
    input: &str,
) -> (
    HashMap<&str, ((usize, usize), (usize, usize))>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut fields = HashMap::new();

    for field in parts[0].split('\n') {
        let split: Vec<&str> = field.split(": ").collect();
        assert_eq!(split.len(), 2);

        let nums: Vec<&str> = split[1]
            .split(" or ")
            .flat_map(|range| range.split('-'))
            .collect();
        assert_eq!(nums.len(), 4);

        let nums: Vec<usize> = nums.iter().map(|&num| num.parse().unwrap()).collect();

        fields.insert(split[0], ((nums[0], nums[1]), (nums[2], nums[3])));
    }

    let your_tickets: Vec<&str> = parts[1].split('\n').skip(1).collect();
    assert_eq!(your_tickets.len(), 1);
    let your_ticket = your_tickets[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    let nearby_tickets = parts[2]
        .split('\n')
        .skip(1)
        .map(|ticket| ticket.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    (fields, your_ticket, nearby_tickets)
}

fn find_invalid_tickets(
    nearby_tickets: &[Vec<usize>],
    fields: &HashMap<&str, ((usize, usize), (usize, usize))>,
) -> Vec<(usize, usize)> {
    let mut invalid_tickets = Vec::new();

    for ticket in nearby_tickets.iter().enumerate() {
        for &num in ticket.1 {
            let mut count = 0;
            for field in fields.values() {
                if (num < field.0 .0 || num > field.0 .1) && (num < field.1 .0 || num > field.1 .1)
                {
                    count += 1;
                } else {
                    break;
                }
            }

            let max = fields.values().count();

            if count == max {
                invalid_tickets.push((ticket.0, num));
            }
        }
    }

    invalid_tickets
}
