use itertools::Itertools;
use std::fs;

fn main() {
    let mut numbers = read_numbers();
    numbers.sort_unstable();

    let valid_combination = numbers
        .iter()
        .combinations(3)
        .find(|x| x.iter().map(|x| **x).sum::<u32>() == 2020 as u32)
        .unwrap();
        println!(
            "And the result is {} which is the product of {}, {} and {}",
            valid_combination.iter().map(|x| **x as u64).product::<u64>(),
            valid_combination[0],
            valid_combination[1],
            valid_combination[2]
        );
}

fn read_numbers() -> Vec<u32> {
    fs::read_to_string("input").expect("Something went wrong reading the file").lines()
        .map(|x| match x.parse::<u32>() {
            Ok(x) => x,
            Err(_) => 0,
        })
        .collect::<Vec<u32>>()
}
