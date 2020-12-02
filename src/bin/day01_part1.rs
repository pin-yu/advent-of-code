use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_input_data(file_name);
    let result = solve(&input_vec.unwrap());

    println!("{}", result);
}

fn read_input_data(file_name: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();

    for line in buffered.lines() {
        vec.push(line?.trim().parse::<i32>().unwrap());
    }

    Ok(vec)
}

fn solve(input_vec: &Vec<i32>) -> Option(i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();

    // this problem is simliar to leet code two sum
    for (idx, num) in input_vec.iter().enumerate() {
        let residual = 2020 - num;

        if !map.contains_key(&residual) {
            map.insert(*num, idx);
        } else {
            return input_vec[idx] * input_vec[*map.get(&residual).unwrap()];
        }
    }

    -1
}