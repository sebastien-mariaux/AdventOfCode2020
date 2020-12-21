use regex::Regex;
use std::collections::BTreeMap;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is '{}'", result);
}

fn solve_puzzle(file_name: &str) -> String {
    let data = read_data(file_name);
    let re = Regex::new(r"(?P<ing>[a-z\s]+) \(contains (?P<allergens>[a-z,\s]+)\)").unwrap();
    let mut foods: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    let mut ingredients_count: HashMap<String, usize> = HashMap::new();
    data.lines().for_each(|line| {
        let caps = re.captures(line).unwrap();
        let words = caps["ing"].to_string();
        let allergens = caps["allergens"].to_string();
        for allergen in allergens.split(", ") {
            let ingredients = words
                .split(' ')
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            let current = foods.get_mut(&allergen.to_string());
            match current {
                Some(x) => {
                    *x = x.intersection(&ingredients).cloned().collect();
                }
                None => {
                    foods.insert(allergen.to_string(), ingredients);
                }
            }
        }
        words.split(' ').map(|x| x.to_string()).for_each(|x| {
            *ingredients_count.entry(x).or_insert(0) += 1;
        });
    });

    let mut new_foods = foods.clone();
    let mut new_map: HashMap<String, String> = HashMap::new();
    while let Some((key, values)) = new_foods.iter().find(|(_, values)| values.len() == 1) {
        let single_value = values.iter().next().unwrap().to_string();
        new_map.insert(key.to_string(), single_value.to_owned());
        let mut new_food: BTreeMap<String, HashSet<String>> = BTreeMap::new();
        new_foods.iter().for_each(|(k, v)| {
            let new_set = v
                .iter()
                .filter(|x| x != &&single_value)
                .map(|x| x.clone())
                .collect::<HashSet<String>>();
            new_food.insert(k.clone(), new_set);
        });
        new_foods = new_food;
    }

    for allergen in new_map.values() {
        let x = ingredients_count.get_mut(&allergen.to_string()).unwrap();
        *x = 0
    }

    let mut output: Vec<String> = Vec::new();
    let mut keys = new_map.keys().cloned().collect::<Vec<String>>();
    keys.sort_unstable();
    for key in keys {
        output.push(new_map.get(&key).unwrap().to_string());
    }

    output.join(",")
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl", solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(
            "kllgt,jrnqx,ljvx,zxstb,gnbxs,mhtc,hfdxb,hbfnkq",
            solve_puzzle("input")
        );
    }
}
