use regex::Regex;
use std::fs;

fn main() {
    let result = solve_puzzle();
    println!("And the result is: {}", result);
}

fn solve_puzzle() -> usize {
    let data = read_lines();
    data.iter().filter(|x| is_valid(x)).count()
}

struct ParsedLine {
    first: u32,
    second: u32,
    letter: char,
    password: String,
}

impl ParsedLine {
    fn new(line: &str) -> ParsedLine {
        let re =
            Regex::new(r"(?P<first>\d+)-(?P<second>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
                .unwrap();
        let caps = re.captures(line).unwrap();
        ParsedLine {
            first: caps["first"].to_string().parse::<u32>().unwrap(),
            second: caps["second"].to_string().parse::<u32>().unwrap(),
            letter: caps["letter"].to_string().chars().next().unwrap(),
            password: caps["password"].to_string(),
        }
    }
}

fn is_valid(line: &&ParsedLine) -> bool {
    let first = line.password.chars().nth(line.first as usize - 1).unwrap();
    let second = line.password.chars().nth(line.second as usize - 1).unwrap();

    match (first == line.letter, second == line.letter) {
        (true, true) => false,
        (true, false) => true,
        (false, true) => true,
        (false, false) => false,
    }
}

fn read_lines() -> Vec<ParsedLine> {
    fs::read_to_string("input")
        .expect("Error")
        .lines()
        .map(|x| ParsedLine::new(x))
        .collect::<Vec<ParsedLine>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(605, solve_puzzle());
    }
}
