use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u64 {
    let data = read_data(file_name);
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string();
    let mut permutations: HashMap<u32, Vec<Vec<char>>> = HashMap::new();
    let mut x_count: u32 = mask.chars().filter(|x| *x == 'X').count() as u32;

    data.lines().for_each(|line| {
        match line.split(" = ").next().unwrap() {
            "mask" => {
                mask = line.replace("mask = ", "");
                x_count = mask.chars().filter(|x| *x == 'X').count() as u32;
                match permutations.get(&x_count) {
                    Some(_) => (),
                    None => {
                        permutations.insert(x_count, get_permutations(x_count));
                    }
                }
            }
            _ => {
                let (addresses, value) =
                    get_values(line, &mask, &permutations.get(&x_count).unwrap());
                addresses.iter().for_each(|address| {
                    memory.insert(read_binary(address.clone()), value);
                })
            }
        }
    });
    memory.values().sum()
}

fn get_values(line: &str, mask: &String, permutations: &Vec<Vec<char>>) -> (HashSet<String>, u64) {
    let re = Regex::new(r"mem\[(?P<key>\d+)\] = (?P<value>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let memory_address = write_binary(caps["key"].to_string().parse::<u64>().unwrap());
    let memory_addresses = apply_mask(memory_address, mask, permutations);
    let decimal_value = caps["value"].to_string().parse::<u64>().unwrap();

    (memory_addresses, decimal_value)
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

fn get_permutations(x_count: u32) -> Vec<Vec<char>> {
    let mut permutations: Vec<Vec<char>> = vec![vec!['0'], vec!['1']];
    for _ in 1..x_count {
        let mut new_permutations = vec![];
        for mut p in permutations {
            let mut duplicate = p.clone();
            p.push('0');
            duplicate.push('1');
            new_permutations.push(duplicate);
            new_permutations.push(p);
        }
        permutations = new_permutations
    }
    permutations
}

fn apply_mask(input: String, mask: &String, permutations: &Vec<Vec<char>>) -> HashSet<String> {
    let mut addresses = HashSet::new();

    permutations.iter().for_each(|permut| {
        let mut iter_comb = permut.iter();
        let new_address = input
            .chars()
            .enumerate()
            .map(|(idx, value)| match mask.chars().nth(idx).unwrap() {
                'X' => {
                    let next_value = iter_comb.next().unwrap();
                    *next_value
                }
                '1' => '1',
                _ => value,
            })
            .collect::<String>();
        addresses.insert(new_address);
    });
    addresses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(208, solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(3706820676200, solve_puzzle("input"));
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
    fn test_apply_mask_two_x() {
        let input = "000000000000000000000000000000101010".to_string();
        let mask = "000000000000000000000000000000X1001X".to_string();
        let mut output = HashSet::new();
        output.insert("000000000000000000000000000000011010".to_string());
        output.insert("000000000000000000000000000000011011".to_string());
        output.insert("000000000000000000000000000000111010".to_string());
        output.insert("000000000000000000000000000000111011".to_string());
        assert_eq!(output, apply_mask(input, &mask, &get_permutations(2)));
    }

    #[test]
    fn test_apply_mask_three_x() {
        let input = "000000000000000000000000000000011010".to_string();
        let mask = "00000000000000000000000000000000X0XX".to_string();
        let mut output = HashSet::new();
        output.insert("000000000000000000000000000000010000".to_string());
        output.insert("000000000000000000000000000000010001".to_string());
        output.insert("000000000000000000000000000000010010".to_string());
        output.insert("000000000000000000000000000000010011".to_string());
        output.insert("000000000000000000000000000000011000".to_string());
        output.insert("000000000000000000000000000000011001".to_string());
        output.insert("000000000000000000000000000000011010".to_string());
        output.insert("000000000000000000000000000000011011".to_string());
        assert_eq!(output, apply_mask(input, &mask, &get_permutations(3)));
    }

    #[test]
    fn test_permutations() {
        assert_eq!(4, get_permutations(2).len());
        assert_eq!(8, get_permutations(3).len());
        assert_eq!(128, get_permutations(7).len());
        assert_eq!(256, get_permutations(8).len());
        assert_eq!(512, get_permutations(9).len());
    }
}
