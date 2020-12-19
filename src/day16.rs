use std::collections::HashMap;

pub fn part1(input: String) {
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
    let your_ticket = your_tickets[0];

    let nearby_tickets: Vec<Vec<usize>> = parts[2].split('\n').skip(1).map(|ticket| ticket.split(',').map(|num| num.parse().unwrap()).collect()).collect();

    let mut invalid_nums = Vec::new();

    for ticket in nearby_tickets {
        for num in ticket {
            let mut count = 0;
           for field in fields.values() {
               if (num < field.0.0 || num > field.0.1) && (num < field.1.0 || num > field.1.1) {
                   count += 1;
               }
           }

            let max = fields.values().count();

            if count == max - 1 {
                invalid_nums.push(num);
            }
        }
    }

    let sum: usize = invalid_nums.iter().sum();
    println!("{}", sum)
}
