use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(&file_name).unwrap();
    println!("part1 result: {}", solve_part1(&input_vec).unwrap());
    println!("part2 result: {}", solve_part2(&input_vec).unwrap());
}

fn read_lines_to_vec(file_name: &String) -> Result<Vec<String>, Error> {
    let input = File::open(file_name).unwrap();
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();
    for line in buffered.lines() {
        vec.push(line?.trim().to_owned());
    }

    Ok(vec)
}

fn solve_part1(input_vec: &[String]) -> Option<i32> {
    let mut max_seat_id = 0;
    for text in input_vec.iter() {
        let row = get_row(text);
        let col = get_col(text);
        let seat_id = row * 8 + col;
        max_seat_id = cmp::max(max_seat_id, seat_id);
    }

    Some(max_seat_id)
}

fn solve_part2(input_vec: &[String]) -> Option<i32> {
    let mut seats: Vec<i32> = Vec::new();
    for text in input_vec.iter() {
        let row = get_row(text);
        let col = get_col(text);
        let seat_id = row * 8 + col;
        seats.push(seat_id);
    }

    find_vacancy_seat(seats)
}

fn get_row(text: &String) -> i32 {
    let row_str = &text[..7].to_owned();
    divider(&row_str, 0, 127, 'F', 'B')
}

fn get_col(text: &String) -> i32 {
    let col_str = &text[7..].to_owned();
    divider(col_str, 0, 7, 'L', 'R')
}

fn divider(
    text: &String,
    mut lower_bound: i32,
    mut upper_bound: i32,
    lower_char: char,
    upper_char: char,
) -> i32 {
    for ch in text.chars() {
        let remaining_rows = upper_bound - lower_bound + 1;
        let cut_point = remaining_rows / 2;

        if ch == lower_char {
            upper_bound -= cut_point;
        } else if ch == upper_char {
            lower_bound += cut_point;
        } else {
            panic!("invalid input!")
        }
    }
    // eventually, lrb will equal urb
    lower_bound
}

fn find_vacancy_seat(mut seats: Vec<i32>) -> Option<i32> {
    seats.sort();

    let mut current_seat_id = seats[0];

    // find the lack seat id
    for &v in seats.iter() {
        if v != current_seat_id {
            return Some(current_seat_id);
        }
        current_seat_id += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let vec: Vec<String> = vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();

        assert_eq!(solve_part1(&vec).unwrap(), 820);
    }

    #[test]
    fn test_solve_part2() {
        let vec: Vec<String> = vec!["FFFFFFFLLL", "FFFFFFFLLR", "FFFFFFFLRL", "FFFFFFFRLL"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();

        assert_eq!(solve_part2(&vec).unwrap(), 3);
    }
}
