use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn read_day01_input_data(file_name: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(file_name)?;
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();

    for line in buffered.lines() {
        vec.push(line?.trim().parse::<i32>().unwrap());
    }

    Ok(vec)
}
