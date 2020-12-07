use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    println!("Oy, oy, oy, the result is {}", run_input(data));
}

fn run_input(data: String) -> u32 {
    let mut rules: BTreeMap<String, HashMap<String, u32>> = BTreeMap::new();
    data.lines().for_each(|line| {
        let (bag, inside_bags) = parse_line(line);
        let containers = rules.entry(bag).or_insert(HashMap::new());
        for (inside_bag, count) in inside_bags {
            let add_bag = containers.entry(inside_bag).or_insert(0);
            *add_bag += count;
        }
    });
    count_for_bag(&"shiny gold".to_string(), &rules)
}

fn count_for_bag(bag_name: &String, rules: &BTreeMap<String, HashMap<String, u32>>) -> u32 {
    let mut count = 0;
    rules
        .get(bag_name)
        .unwrap()
        .iter()
        .for_each(|(name, number)| count += number + number * count_for_bag(name, rules));
    count
}

fn parse_line(line: &str) -> (String, HashMap<String, u32>) {
    let re = Regex::new(r"(\d )?([a-z]+ [a-z]+) bag").unwrap();
    let mut inside_bags: HashMap<String, u32> = HashMap::new();
    let mut bags = re.captures_iter(line);
    let big_bag = &bags.next().unwrap()[2];
    for cap in bags {
        let count = match cap.get(1) {
            Some(x) => x.as_str().trim().parse::<u32>().unwrap(),
            None => continue,
        };
        inside_bags.insert(cap[2].to_string(), count);
    }

    (big_bag.to_string(), inside_bags)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main_process() {
        let entry_data: String = String::from(
            "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.",
        );
        assert_eq!(126, run_input(entry_data));
    }

    #[test]
    fn test_main_process_with_multiple_bags_inside() {
        let entry_data: String = String::from(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.",
        );
        assert_eq!(32, run_input(entry_data));
    }

    #[test]
    fn test_parse_line_with_two_inside() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let mut inside_bags = HashMap::new();
        inside_bags.insert(String::from("bright white"), 1);
        inside_bags.insert(String::from("muted yellow"), 2);

        let expected: (String, HashMap<String, u32>) = (String::from("light red"), inside_bags);
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_parse_line_with_one_inside() {
        let line = "bright white bags contain 1 shiny gold bag.";
        let mut inside_bags = HashMap::new();
        inside_bags.insert(String::from("shiny gold"), 1);

        let expected: (String, HashMap<String, u32>) = (String::from("bright white"), inside_bags);
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_parse_line_with_nothing_inside() {
        let line = "dark violet bags contain no other bags.";
        let inside_bags = HashMap::new();

        let expected: (String, HashMap<String, u32>) = (String::from("dark violet"), inside_bags);
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_parse_line_with_four_inside() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags, 5 faded blue bags, 6 dotted black bags.";
        let mut inside_bags = HashMap::new();
        inside_bags.insert(String::from("bright white"), 1);
        inside_bags.insert(String::from("muted yellow"), 2);
        inside_bags.insert(String::from("faded blue"), 5);
        inside_bags.insert(String::from("dotted black"), 6);

        let expected: (String, HashMap<String, u32>) = (String::from("light red"), inside_bags);
        assert_eq!(expected, parse_line(line));
    }
}
