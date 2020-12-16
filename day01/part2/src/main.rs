use itertools::Itertools;
use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u64 {
    let data = read_data(file_name);
    let mut numbers = data
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    numbers.sort_unstable();
    numbers
        .iter()
        .combinations(3)
        .find(|x| x.iter().map(|x| **x).sum::<u32>() == 2020 as u32)
        .unwrap()
        .iter()
        .map(|x| **x as u64)
        .product::<u64>()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(244300320, solve_puzzle("input"));
    }
}
