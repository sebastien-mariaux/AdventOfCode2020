use std::{fs, vec};
use std::collections::BTreeMap;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    println!("oy, oy, oy, the result is {}", run_input(data));
}

fn run_input(data: String) -> u32 {
    let mut rules:BTreeMap<String, Vec<String>> = BTreeMap::new();
    data.lines().filter(|line| !line.contains("no other bag")).for_each(|line| {
        parse_line(line).iter().for_each(|(key, value)| {
            let containers = rules.entry(key.clone()).or_insert(vec![]);
            containers.push(value.clone());
        })
    });
    let valid_bags: HashSet<String> = HashSet::new();
    count_for_bag(&"shiny gold".to_string(), &rules, valid_bags).len() as u32
}

fn count_for_bag(bag_name: &String, rules: &BTreeMap<String, Vec<String>>, mut valid_bags: HashSet<String>) -> HashSet<String>{
    valid_bags = match rules.get(bag_name) {
        Some(bags) => {
            bags.iter().for_each(|bag| {
                valid_bags.insert(bag.to_string());
             });
            for bag in bags {
                valid_bags = count_for_bag(bag, rules, valid_bags);
            }
            valid_bags
        },
        None => valid_bags,
    };
    valid_bags
}

fn parse_line(line: &str) -> Vec<(String, String)> {
    let re =  Regex::new(r"([a-z]+ [a-z]+) bag").unwrap();
    let mut output:Vec<(String, String)> = vec![];
    let mut bags = re.captures_iter(line);
    let big_bag = &bags.next().unwrap()[1];
    bags.for_each(|cap| {
        output.push((cap[1].to_string(), big_bag.to_string()));

    });
    output
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main_process() {
        let entry_data: String = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.");
        assert_eq!(4, run_input(entry_data));
    }

    #[test]
    fn test_parse_line_with_two_inside() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected = vec![("bright white".to_string(), "light red".to_string()), ("muted yellow".to_string(), "light red".to_string())];
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_parse_line_with_one_inside() {
        let line = "bright white bags contain 1 shiny gold bag.";
        let expected = vec![("shiny gold".to_string(), "bright white".to_string())];
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_parse_line_with_four_inside() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags, 5 faded blue bags, 6 dotted black bags.";
        let expected = vec![("bright white".to_string(), "light red".to_string()), ("muted yellow".to_string(), "light red".to_string()), 
                                            ("faded blue".to_string(), "light red".to_string()), ("dotted black".to_string(), "light red".to_string())];
        assert_eq!(expected, parse_line(line));

    }
}
