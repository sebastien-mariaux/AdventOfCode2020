use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

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

fn solve_puzzle(file_name: &str) -> u128 {
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

    let own_ticket = iter_data
        .nth(1)
        .unwrap()
        .split(',')
        .map(|value| value.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let mut possibilities: HashMap<String, HashSet<usize>> = HashMap::new();
    let full_vec: HashSet<usize> = (0..rules.len()).collect();
    rules.iter().for_each(|rule| {
        possibilities.insert(rule.name.clone(), full_vec.clone());
    });
    iter_data
        .skip(2)
        .filter(|line| {
            line.split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .all(|value| rules.iter().any(|rule| rule.valid_value(&value)))
        })
        .for_each(|line| {
            line.split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(idx, value)| {
                    rules.iter().for_each(|rule| {
                        if !rule.valid_value(&value) {
                            let set = possibilities.get_mut(&rule.name);
                            if let Some(x) = set {
                                x.remove(&idx);
                            }
                        }
                    })
                })
        });

    let mut single_possibilities: HashMap<String, usize> = HashMap::new();
    while let Some((key, values)) = possibilities.iter().find(|(_, values)| values.len() == 1) {
            let (name, value) = (key.clone(), *values.iter().next().unwrap());
        possibilities.iter_mut().for_each(|(_, values)| {
            values.remove(&value);
        });
        single_possibilities.insert(name, value);
    }

    single_possibilities
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .fold(1, |acc, (_, v)| acc * own_ticket[*v])
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(3173135507987, solve_puzzle("input"));
    }
}
