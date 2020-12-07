use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

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
    solve(input_vec, 3, 1)
}

fn solve_part2(input_vec: &[String]) -> Option<i64> {
    let mut result: i64 = 1;
    for h in (1..=7).step_by(2) {
        result *= solve(input_vec, h, 1).unwrap() as i64;
    }
    result *= solve(input_vec, 1, 2).unwrap() as i64;
    Some(result)
}

fn solve(input_vec: &[String], horizontal_step: usize, vertical_step: usize) -> Option<i32> {
    let line_len: usize = input_vec[0].len();
    let mut pos = line_len;

    let mut tree = 0;
    for line in input_vec[vertical_step..].iter().step_by(vertical_step) {
        pos = next_horizontal_pos(pos, line_len, horizontal_step);

        if line.chars().nth(pos).unwrap() == '#' {
            tree += 1;
        }
    }
    Some(tree)
}

fn next_horizontal_pos(current_pos: usize, line_len: usize, step: usize) -> usize {
    let pos = current_pos + step;

    pos.rem_euclid(line_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let vec: Vec<String> = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(solve_part1(&vec).unwrap(), 7);
    }

    #[test]
    fn test_solve_part2() {
        let vec: Vec<String> = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();

        assert_eq!(solve_part2(&vec).unwrap(), 336);
    }
}
