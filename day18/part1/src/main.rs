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
    let mut next_operation = '+';
    expression
        .replace(")", "")
        .replace("(", "")
        .split(' ')
        .fold(0, |result, value| match value {
            "+" => {
                next_operation = '+';
                result
            }
            "*" => {
                next_operation = '*';
                result
            }
            _ => {
                let number = value.parse::<u64>().unwrap();
                match next_operation {
                    '+' => result + number,
                    '*' => result * number,
                    _ => panic!("Qu'es aquo ?"),
                }
            }
        })
}

fn compute(expression: &str) -> u64 {
    let mut text = expression.to_string();
    // println!("{}", text);
    let re = Regex::new(r"(?P<segment>\([\d\s\+\*]*\))").unwrap();
    while let Some(_) = text.chars().find(|x| *x == '(' || *x == ')') {
        for cap in re.captures_iter(&text.clone()) {
            let segment = cap["segment"].to_string();
            // println!("cap {}", segment);
            text = text.replace(&segment, &compute_simple(&segment).to_string());
            // println!("{}", text);
        }
    }

    compute_simple(&text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(7293529867931, solve_puzzle("input"));
    }

    #[test]
    fn test_basic_example() {
        assert_eq!(71, compute("1 + 2 * 3 + 4 * 5 + 6"));
    }

    #[test]
    fn test_example_with_parenthesis() {
        assert_eq!(26, compute("2 * 3 + (4 * 5)"));
    }

    #[test]
    fn test_other_example_with_parenthesis() {
        assert_eq!(437, compute("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    }

    #[test]
    fn test_example_with_nested_parenthesis() {
        assert_eq!(12240, compute("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    }

    #[test]
    fn test_example_with_multiple_nested_parenthesis() {
        assert_eq!(
            13632,
            compute("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
