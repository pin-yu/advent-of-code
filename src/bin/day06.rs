use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(&file_name).unwrap();
    println!("part1 result: {}", solve_part1(&input_vec));
    println!("part2 result: {}", solve_part2(&input_vec));
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

fn solve_part1(input_vec: &[String]) -> usize {
    let mut sum = 0;

    // init a temp_text
    let mut temp_text = "".to_owned();

    for line in input_vec.iter() {
        if line != "" {
            temp_text += line;
        } else {
            sum += group_count(&temp_text);

            // clean the temp text
            temp_text = "".to_owned();
        }
    }

    // remember to count the last group
    sum += group_count(&temp_text);

    sum
}

fn solve_part2(input_vec: &[String]) -> usize {
    let mut sum = 0;

    // init a temp_text
    let mut temp_text = "".to_owned();
    let mut people_in_a_group = 0;
    for line in input_vec.iter() {
        if line != "" {
            people_in_a_group += 1;
            temp_text += line;
        } else {
            sum += everyone_count(&temp_text, people_in_a_group);

            // clean the temp text
            temp_text = "".to_owned();
            people_in_a_group = 0;
        }
    }

    // remember to count the last group
    sum += everyone_count(&temp_text, people_in_a_group);

    sum
}

fn group_count(text: &str) -> usize {
    let mut text_set = HashSet::new();

    for ch in text.chars() {
        text_set.insert(ch);
    }

    text_set.len()
}

fn everyone_count(text: &str, people_num: usize) -> usize {
    let mut char_hash = HashMap::new();

    let mut everyone_say_yes_num = 0;
    for ch in text.chars() {
        let counter = char_hash.entry(ch).or_insert(0);
        *counter += 1;

        if *counter == people_num {
            everyone_say_yes_num += 1;
        }
    }

    everyone_say_yes_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let vec: Vec<String> = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(solve_part1(&vec), 11);
        assert_eq!(solve_part2(&vec), 6);
    }
}
