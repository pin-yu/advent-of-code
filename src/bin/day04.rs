use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(file_name).unwrap();
    println!("part1 result: {}", solve_part1(input_vec).unwrap());
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

fn solve_part1(input_vec: Vec<String>) -> Option<i32> {
    let batch_vec = to_batch(input_vec).unwrap();

    let mut valid = 0;
    for passport in batch_vec.iter() {
        if is_valid_passport(&passport) {
            valid += 1;
        }
    }
    Some(valid)
}

fn is_valid_passport(passport: &String) -> bool {
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

fn to_batch(input_vec: Vec<String>) -> Option<Vec<String>> {
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
    fn test_solve() {
        let vec: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
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

        assert_eq!(solve_part1(vec).unwrap(), 2);
    }
}
