use std::collections::{BTreeMap, HashSet};
use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);
    let mut split_data = data.split("\n\n");
    let data_rules = split_data.next().unwrap().to_string().replace("\"", "");
    let data_messages = split_data.next().unwrap().to_string();
    let rules = data_rules
        .lines()
        .map(|rule| {
            let mut split = rule.split(": ");
            let index = split.next().unwrap().parse::<usize>().unwrap();
            let value = split.next().unwrap().to_string();
            (index, value)
        })
        .collect::<BTreeMap<usize, String>>();
    let matches = compute_rule(0, &rules);
    
    data_messages.lines().filter(|message| matches.contains(message as &str)).count() 
}

fn compute_rule(index: usize, rules: &BTreeMap<usize, String>) -> HashSet<String> {
    let rule = rules.get(&index).unwrap();
    println!("Rule {}", rule);
    let mut matches: HashSet<String> = HashSet::new();
    matches = match rule.as_str() {
        "a"|"b" => {
            matches.insert(rule.to_string());
            matches
        },
        _ => {
            matches.insert(String::from(""));
            for number in rule.split(' ').map(|x| x.parse::<usize>().unwrap()) {
                let next_rules = compute_rule(number, rules);
                let mut new_matches = HashSet::new();
                for current_rule in &matches {
                    println!("Current rule {}", current_rule);
                    for next_rule in &next_rules {
                        let new_rule = format!("{}{}", current_rule, next_rule);
                        new_matches.insert(new_rule);
                    }
                }
                matches = new_matches;
            };
            matches
        }
    };
    println!("Rule {} Matches{:?}", rule, matches);
    matches
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2, solve_puzzle("example_data"));
    }

    #[test]
    fn test_simple_example() {
        assert_eq!(1, solve_puzzle("simple_example"));
    }

    #[test]
    fn test_input() {
        assert_eq!(1, solve_puzzle("input"));
    }
}
