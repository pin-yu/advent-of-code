use std::env;

use utils::read_input_data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input_vec = read_input_data(file_name);
    let result = solve(&input_vec.unwrap());

    println!("{}", result.unwrap());
}

fn solve(input_vec: &Vec<i32>) -> Option<i32> {
    for num1 in input_vec.iter() {
        for num2 in input_vec.iter() {
            if num2 == num1 {
                continue;
            }

            let residual = 2020 - num1 - num2;
            if input_vec.contains(&residual) {
                return Some(residual * num1 * num2);
            }
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
        assert_eq!(solve(&numbers).unwrap(), 241861950);
    }
}
