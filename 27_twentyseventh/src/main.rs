use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u64 {
    let data = read_data(file_name);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string();

    data.lines().for_each(|line| {
        match line.split(" = ").next().unwrap() {
            "mask" => {
                mask = line.replace("mask = ", "");
            },
            _ => {
                let (key, value) = get_values(line, &mask);
                memory.insert(key, value);
            },
        }
    });
    memory.values().sum()
}

fn get_values(line: &str, mask: &String) -> (u64, u64) {
    let re = Regex::new(r"mem\[(?P<key>\d+)\] = (?P<value>\d+)")
            .unwrap();
    let caps = re.captures(line).unwrap();
    let memory_address = caps["key"].to_string().parse::<u64>().unwrap();
    let decimal_value = caps["value"].to_string().parse::<u64>().unwrap();
    let masked_value = apply_mask(write_binary(decimal_value), &mask);
    (memory_address, read_binary(masked_value))
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

fn write_binary(decimal: u64) -> String {
    format!("{:036b}", decimal)
}

fn read_binary(binary: String) -> u64 {
    binary
        .chars()
        .rev()
        .enumerate()
        .fold(0, |result, (idx, value)| match value {
            '1' => result + 2u64.pow(idx as u32),
            _ => result,
        })
}

fn apply_mask(input: String, mask: &String) -> String {
    input
        .chars()
        .enumerate()
        .map(|(idx, value)| match mask.chars().nth(idx).unwrap() {
            '1' => '1',
            '0' => '0',
            _ => value,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(165, solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(14925946402938, solve_puzzle("input"));
    }

    #[test]
    fn test_write_binary() {
        assert_eq!(
            "000000000000000000000000000000001011".to_string(),
            write_binary(11)
        );
        assert_eq!(
            "000000000000000000000000000001100101".to_string(),
            write_binary(101)
        );
        assert_eq!(
            "000000000000000000000000000001001001".to_string(),
            write_binary(73)
        );
    }

    #[test]
    fn test_read_binary() {
        assert_eq!(
            64,
            read_binary("000000000000000000000000000001000000".to_string())
        );
        assert_eq!(
            101,
            read_binary("000000000000000000000000000001100101".to_string())
        );
        assert_eq!(
            73,
            read_binary("000000000000000000000000000001001001".to_string())
        );
    }

    #[test]
    fn test_apply_mask() {
        let input = "000000000000000000000000000000001011".to_string();
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();
        let output = "000000000000000000000000000001001001".to_string();
        assert_eq!(output, apply_mask(input, &mask));
    }

    #[test]
    fn test_apply_mask_second() {
        let input = "000000000000000000000000000000000000".to_string();
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();
        let output = "000000000000000000000000000001000000".to_string();
        assert_eq!(output, apply_mask(input, &mask));
    }

    #[test]
    fn test_apply_mask_no_effect() {
        let input = "000000000000000000000000000001100101".to_string();
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string();
        let output = "000000000000000000000000000001100101".to_string();
        assert_eq!(output, apply_mask(input, &mask));
    }
}
