use regex::Regex;
use std::collections::BTreeMap;
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
    let mut rules = data_rules
        .lines()
        .map(|rule| {
            let mut split = rule.split(": ");
            let index = split.next().unwrap().to_string();
            let value = split.next().unwrap().to_string();
            (index, value)
        })
        .collect::<BTreeMap<String, String>>();

    // Update rules for part 2
    let rule = rules.get_mut("8").unwrap();
    *rule = "42 | 42 8".to_string();

    let rule = rules.get_mut("11").unwrap();
    *rule = "42 31 | 42 11 31".to_string();

    let mut main_rule = rules.get("0").unwrap().clone();
    main_rule = format!(" {} ", main_rule);
    loop {
        if main_rule.chars().all(|x| !x.is_numeric()) {
            break;
        };
        main_rule.clone().split(' ').for_each(|x| {
            if x.chars().all(|c| c.is_numeric()) {
                if let Some(y) = rules.get(x) {
                    main_rule = main_rule.replace(&format!(" {} ", x), &format!(" ( {} ) ", y));
                };
            }
        });
    }
    main_rule = format!("^{}$", main_rule.replace(" ", ""));
    println!("{}", main_rule);

    let re = Regex::new(&main_rule).unwrap();
    data_messages.lines().filter(|x| re.is_match(x)).count()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(12, solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(279, solve_puzzle("input"));
    }
}
