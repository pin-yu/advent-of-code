use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_lines_to_vec(file_name);
    let result = solve(&input_vec.unwrap());

    println!("{}", result.unwrap());
}

pub fn read_lines_to_vec(file_name: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();

    for line in buffered.lines() {
        vec.push(line?.trim().parse::<i32>().unwrap());
    }

    Ok(vec)
}

fn solve(input_vec: &[i32]) -> Option<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    // this problem is simliar to leet code two sum
    for (idx, &num1) in input_vec.iter().enumerate() {
        let residual = 2020 - num1;

        if !map.contains_key(&residual) {
            map.insert(num1, idx);
        } else {
            let num2 = input_vec[*map.get(&residual).unwrap()];
            return Some(num1 * num2);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve(&numbers).unwrap(), 514579);
    }
}
