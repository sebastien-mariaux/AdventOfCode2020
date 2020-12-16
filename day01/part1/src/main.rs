use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut numbers = data
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    numbers.sort_unstable();
    loop {
        let number = numbers.pop().unwrap();
        let target = 2020 - number;

        if numbers.iter().any(|x| x == &target) {
            break target * number;
        }
    }
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(319531, solve_puzzle("input"));
    }
}
