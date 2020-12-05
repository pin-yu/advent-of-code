use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(file_name).unwrap();

    let result = solve(&input_vec).unwrap();
    println!("{}", result);
}

pub fn read_lines_to_vec(file_name: &str) -> Result<Vec<String>, Error> {
    let input = File::open(file_name).unwrap();
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();

    for line in buffered.lines() {
        vec.push(line?.trim().to_owned());
    }

    Ok(vec)
}

pub fn solve(input_vec: &[String]) -> Option<i32> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let mut counter: i32 = 0;

    for line in input_vec.iter() {
        let caps = re.captures(line).unwrap();

        let pos1: usize = caps[1].parse::<usize>().unwrap() - 1;
        let pos2: usize = caps[2].parse::<usize>().unwrap() - 1;

        let ch = &mut caps[3].chars().next().unwrap();

        let text = &caps[4];
        let pos1_letter = text.chars().nth(pos1).unwrap();
        let pos2_letter = text.chars().nth(pos2).unwrap();

        if pos1_letter == *ch && pos2_letter != *ch {
            counter += 1;
        }

        if pos2_letter == *ch && pos1_letter != *ch {
            counter += 1;
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let lines: Vec<String> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();

        assert_eq!(solve(&lines).unwrap(), 1);
    }
}
