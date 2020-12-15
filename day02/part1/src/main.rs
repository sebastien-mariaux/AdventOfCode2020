use regex::Regex;
use std::fs;

struct ParsedLine {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

impl ParsedLine {
    fn new(line: &str) -> ParsedLine {
        let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)")
            .unwrap();
        let caps = re.captures(line).unwrap();
        ParsedLine {
            min: caps["min"].to_string().parse::<u32>().unwrap(),
            max: caps["max"].to_string().parse::<u32>().unwrap(),
            letter: caps["letter"].to_string().chars().next().unwrap(),
            password: caps["password"].to_string(),
        }
    }
}

fn is_valid(line: &&ParsedLine) -> bool {
    let frequency = line
        .password
        .chars()
        .filter(|letter| letter == &line.letter)
        .count();
    frequency >= line.min as usize && frequency <= line.max as usize
}

fn read_lines() -> Vec<ParsedLine> {
    fs::read_to_string("input")
        .expect("Error")
        .lines()
        .map(|x| ParsedLine::new(x))
        .collect::<Vec<ParsedLine>>()
}

fn main() {
    let data = read_lines();
    let result = data.iter().filter(|x| is_valid(x)).count();
    println!("And the result is: {}", result);
}
