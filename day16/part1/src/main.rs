use std::fs;
// use std::collections::HashMap;
use regex::Regex;

struct Rule {
    name: String,
    min_1: u32,
    max_1: u32,
    min_2: u32,
    max_2: u32,
}

impl Rule {
    fn valid_value(&self, value: &u32) -> bool {
        value >= &self.min_1 && value <= &self.max_1 || value >= &self.min_2 && value <= &self.max_2
    }
}

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);
    let mut iter_data = data.lines();
    let mut rules = Vec::new();
    let rule_regex =
        Regex::new(r"(?P<rule>.*): (?P<min_1>\d+)-(?P<max_1>\d+) or (?P<min_2>\d+)-(?P<max_2>\d+)")
            .unwrap();

    loop {
        match iter_data.next().unwrap() {
            "" => break,
            line => {
                let caps = rule_regex.captures(line).unwrap();
                let rule = Rule {
                    name: caps["rule"].to_string(),
                    min_1: caps["min_1"].to_string().parse::<u32>().unwrap(),
                    max_1: caps["max_1"].to_string().parse::<u32>().unwrap(),
                    min_2: caps["min_2"].to_string().parse::<u32>().unwrap(),
                    max_2: caps["max_2"].to_string().parse::<u32>().unwrap(),
                };
                rules.push(rule);
            }
        }
    }

    iter_data
        .skip(4)
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .filter(|value| rules.iter().all(|rule| !rule.valid_value(&value)))
        })
        .flatten()
        .sum::<u32>()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(71, solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(27870, solve_puzzle("input"));
    }
}
