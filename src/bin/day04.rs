use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(file_name).unwrap();
    println!("part1 result: {}", solve_part1(&input_vec).unwrap());
    println!("part2 result: {}", solve_part2(&input_vec).unwrap());
}

fn read_lines_to_vec(file_name: &str) -> Result<Vec<String>, Error> {
    let input = File::open(file_name).unwrap();
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();
    for line in buffered.lines() {
        vec.push(line?.trim().to_owned());
    }

    Ok(vec)
}

fn solve_part1(input_vec: &[String]) -> Option<i32> {
    let batch_vec = to_batch(input_vec).unwrap();

    let mut valid = 0;
    for passport in batch_vec.iter() {
        if no_missing_field(&passport) {
            valid += 1;
        }
    }
    Some(valid)
}

fn solve_part2(input_vec: &[String]) -> Option<i32> {
    let batch_vec = to_batch(input_vec).unwrap();

    let mut valid = 0;
    for passport in batch_vec.iter() {
        if is_valid_field(&passport) {
            valid += 1;
        }
    }
    Some(valid)
}

fn is_valid_field(passport: &String) -> bool {
    if !no_missing_field(passport) {
        return false;
    }

    if valid_byr(passport)
        && valid_iyr(passport)
        && valid_eyr(passport)
        && valid_hgt(passport)
        && valid_hcl(passport)
        && valid_ecl(passport)
        && valid_pid(passport)
    {
        true
    } else {
        false
    }
}

fn valid_byr(passport: &String) -> bool {
    let re = Regex::new(r"byr:(\d+)").unwrap();
    let caps = match re.captures(passport) {
        Some(x) => x,
        None => return false,
    };

    let byr = caps[1].parse::<i32>().unwrap();
    if byr < 1920 || byr > 2002 {
        return false;
    }
    true
}

fn valid_iyr(passport: &String) -> bool {
    let re = Regex::new(r"iyr:(\d+)").unwrap();
    let caps = match re.captures(passport) {
        Some(x) => x,
        None => return false,
    };

    let iyr = caps[1].parse::<i32>().unwrap();
    if iyr < 2010 || iyr > 2020 {
        return false;
    }
    true
}

fn valid_eyr(passport: &String) -> bool {
    let re = Regex::new(r"eyr:(\d+)").unwrap();
    let caps = match re.captures(passport) {
        Some(x) => x,
        None => return false,
    };

    let eyr = caps[1].parse::<i32>().unwrap();
    if eyr < 2020 || eyr > 2030 {
        return false;
    }
    true
}

fn valid_hgt(passport: &String) -> bool {
    let re = Regex::new(r"hgt:(\d+)([a-z]+)").unwrap();
    let caps = match re.captures(passport) {
        Some(x) => x,
        None => return false,
    };

    let hgt = caps[1].parse::<i32>().unwrap();
    let unit = &caps[2];

    if unit == "in" {
        if hgt < 59 || hgt > 76 {
            return false;
        }
    } else if unit == "cm" {
        if hgt < 150 || hgt > 193 {
            return false;
        }
    }
    true
}

fn valid_hcl(passport: &String) -> bool {
    let re = Regex::new(r"hcl:(#[0-9a-f]{6})\b").unwrap();
    match re.captures(passport) {
        Some(_x) => return true,
        None => return false,
    };
}

fn valid_ecl(passport: &String) -> bool {
    let re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    match re.captures(passport) {
        Some(_x) => return true,
        None => return false,
    };
}

fn valid_pid(passport: &String) -> bool {
    let re = Regex::new(r"pid:(\d{9})\b").unwrap();
    match re.captures(passport) {
        Some(_x) => return true,
        None => return false,
    };
}

fn no_missing_field(passport: &String) -> bool {
    // add a space in front of all patterns
    // because I assume there may be identical patterns in the value
    if !passport.contains("byr:") {
        return false;
    } else if !passport.contains("iyr:") {
        return false;
    } else if !passport.contains("eyr:") {
        return false;
    } else if !passport.contains("hgt:") {
        return false;
    } else if !passport.contains("hcl:") {
        return false;
    } else if !passport.contains("ecl:") {
        return false;
    } else if !passport.contains("pid:") {
        return false;
    }
    true
}

fn to_batch(input_vec: &[String]) -> Option<Vec<String>> {
    let mut batch_vec: Vec<String> = Vec::new();

    let mut batch_str = String::new();
    for line in input_vec.iter() {
        if line != "" {
            batch_str.push_str(" ");
            batch_str.push_str(line);
        } else {
            batch_vec.push(batch_str);
            batch_str = "".to_owned();
        }
    }
    // remember to push last batch into vector!
    batch_vec.push(batch_str);

    Some(batch_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let vec: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(solve_part1(&vec).unwrap(), 3);
    }

    #[test]
    fn test_solve_part2() {
        let valid_vec: Vec<String> = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        let invalid_vec: Vec<String> = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(solve_part2(&valid_vec).unwrap(), 4);
        assert_eq!(solve_part2(&invalid_vec).unwrap(), 0);
    }
}
