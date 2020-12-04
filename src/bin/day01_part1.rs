use std::collections::HashMap;
use std::env;

use utils::read_day01_input_data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_day01_input_data(file_name);
    let result = solve(&input_vec.unwrap());

    println!("{}", result.unwrap());
}

fn solve(input_vec: &[i32]) -> Option<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    // this problem is simliar to leet code two sum
    for (idx, num) in input_vec.iter().enumerate() {
        let residual = 2020 - num;

        if !map.contains_key(&residual) {
            map.insert(*num, idx);
        } else {
            return Some(input_vec[idx] * input_vec[*map.get(&residual).unwrap()]);
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
