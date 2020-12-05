use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(file_name).unwrap();
    println!("{}", solve(&input_vec).unwrap());
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

fn solve(input_vec: &[String]) -> Option<i32> {
    let line_len: usize = input_vec[0].len();
    let mut pos = line_len;

    let mut tree = 0;
    for line in input_vec[1..].iter() {
        pos = next_pos(pos, line_len);

        if line.chars().nth(pos).unwrap() == '#' {
            tree += 1;
        }
    }

    Some(tree)
}

fn next_pos(current_pos: usize, line_len: usize) -> usize {
    let pos = current_pos + 3;

    pos.rem_euclid(line_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
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

        assert_eq!(solve(&vec).unwrap(), 7);
    }
}
