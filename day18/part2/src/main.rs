use regex::Regex;
use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u64 {
    let data = read_data(file_name);
    data.lines().fold(0, |sum, line| sum + compute(line))
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

fn compute_simple(expression: &str) -> u64 {
    let mut text = expression.replace(")", "").replace("(", "").to_string();
    let re = Regex::new(r"(?P<segment>\d+( \+ \d+)+)").unwrap();
    for cap in re.captures_iter(&text.clone()) {
        let segment = cap["segment"].to_string();
        let addition = segment
            .split(" + ")
            .fold(0, |acc, number| acc + number.parse::<u64>().unwrap());
        text = text.replacen(&segment, &addition.to_string(), 1);
    }
    text.split(" * ")
        .fold(1, |acc, number| acc * number.parse::<u64>().unwrap())
}

fn compute(expression: &str) -> u64 {
    let mut text = expression.to_string();
    let re = Regex::new(r"(?P<segment>\([\d\s\+\*]*\))").unwrap();
    while text.chars().find(|x| *x == '(' || *x == ')').is_some() {
        for cap in re.captures_iter(&text.clone()) {
            let segment = cap["segment"].to_string();
            text = text.replace(&segment, &compute_simple(&segment).to_string());
        }
    }

    compute_simple(&text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(60807587180737, solve_puzzle("input"));
    }

    #[test]
    fn test_basic_example() {
        assert_eq!(51, compute("1 + (2 * 3) + (4 * (5 + 6))"));
    }

    #[test]
    fn test_example_with_parenthesis() {
        assert_eq!(46, compute("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn test_other_example_with_parenthesis() {
        assert_eq!(1445, compute("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn test_example_with_nested_parenthesis() {
        assert_eq!(669060, compute("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    }

    #[test]
    fn test_example_with_multiple_nested_parenthesis() {
        assert_eq!(
            23340,
            compute("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
