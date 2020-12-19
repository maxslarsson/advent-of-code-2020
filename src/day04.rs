use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::panic;

fn setup(input: &str) -> Vec<HashMap<&str, &str>> {
    // let mut passports: Vec<Vec<HashMap<String, String>>> =
    //     Vec::with_capacity(input.split("\n\n").count());

    let mut passports = Vec::new();

    for passport in input.split("\n\n") {
        passports.push(HashMap::new());
        for field in passport.split([' ', '\n'].as_ref()) {
            let pair: Vec<&str> = field.split(':').collect();
            passports.last_mut().unwrap().insert(pair[0], pair[1]);
        }
    }

    passports
}

pub fn part1(input: String) {
    let input = setup(&input);

    let mut valid_passports = 0;

    for passport in input {
        if passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid")) {
            valid_passports += 1;
        }
    }

    println!("{}", valid_passports);
}

pub fn part2(input: String) {
    let input = setup(&input);

    panic::set_hook(Box::new(|_info| {
        // supresses the panic messages
        // does nothing
    }));

    let mut valid_passports = 0;

    lazy_static! {
        static ref HEX_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref NINE_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    };

    for passport in input {
        let res = panic::catch_unwind(|| {
            let byr = passport.get("byr").unwrap().parse().unwrap();

            if (1920..=2002).contains(&byr) {
                let iyr = passport.get("iyr").unwrap().parse().unwrap();

                if (2010..=2020).contains(&iyr) {
                    let eyr = passport.get("eyr").unwrap().parse().unwrap();

                    if (2020..=2030).contains(&eyr) {
                        let hgt = passport.get("hgt").unwrap();
                        let (hgt, units) = hgt.split_at(hgt.len() - 2);
                        let hgt = hgt.parse().unwrap();

                        if ((units == "cm" && 150 <= hgt && hgt <= 193)
                            || (units == "in" && 59 <= hgt && hgt <= 76))
                            && HEX_RE.is_match(passport.get("hcl").unwrap())
                        {
                            let ecl = passport.get("ecl").unwrap();

                            if (*ecl == "amb"
                                || *ecl == "blu"
                                || *ecl == "brn"
                                || *ecl == "gry"
                                || *ecl == "grn"
                                || *ecl == "hzl"
                                || *ecl == "oth")
                                && NINE_RE.is_match(passport.get("pid").unwrap())
                            {
                                return 1;
                            }
                        }
                    }
                }
            }

            0
        });

        if let Ok(res) = res {
            valid_passports += res;
        }
    }

    println!("{}", valid_passports);
}
