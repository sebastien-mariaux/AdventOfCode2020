use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let data = fs::read_to_string("input").expect("Error");
    let result = data
        .split("\n\n")
        .map(|x| x.to_string().replace("\n", " "))
        .filter(|passport| is_valid(passport, &required_fields))
        .count();
    println!("The result is: {}", result);
}

fn is_valid(passport: &str, required_fields: &[&str]) -> bool {
    if required_fields
        .iter()
        .any(|field| !passport.contains(field))
    {
        return false;
    };

    let mut data = HashMap::new();
    passport.to_string().split(' ').for_each(|segment| {
        let split_segment: Vec<String> = segment
            .to_string()
            .split(':')
            .map(|x| x.to_string())
            .collect();
        data.insert(split_segment[0].clone(), split_segment[1].clone());
    });

    valid_byr(&data)
        && valid_iyr(&data)
        && valid_eyr(&data)
        && valid_height(&data)
        && valid_hcl(&data)
        && valid_ecl(&data)
        && valid_pid(&data)
}

fn valid_pid(passport: &HashMap<String, String>) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(&passport.get("pid").unwrap())
}

fn valid_ecl(passport: &HashMap<String, String>) -> bool {
    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .contains(&passport.get("ecl").unwrap().as_str())
}

fn valid_hcl(passport: &HashMap<String, String>) -> bool {
    let re = Regex::new(r"^#[a-z\d]{6}$").unwrap();
    re.is_match(passport.get("hcl").unwrap())
}

fn valid_height(passport: &HashMap<String, String>) -> bool {
    let height = passport.get("hgt").unwrap();
    let re_cm = Regex::new(r"^\d{3}cm$").unwrap();
    let re_in = Regex::new(r"^\d{2}in$").unwrap();
    if re_cm.is_match(height) {
        let value = height
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<u8>()
            .unwrap();
        value >= 150 && value <= 193
    } else if re_in.is_match(height) {
        let value = height
            .chars()
            .take(2)
            .collect::<String>()
            .parse::<u8>()
            .unwrap();
        value >= 59 && value <= 76
    } else {
        false
    }
}

fn valid_byr(passport: &HashMap<String, String>) -> bool {
    check_year(passport.get("byr").unwrap(), 1920, 2002)
}

fn valid_iyr(passport: &HashMap<String, String>) -> bool {
    check_year(passport.get("iyr").unwrap(), 2010, 2020)
}
fn valid_eyr(passport: &HashMap<String, String>) -> bool {
    check_year(passport.get("eyr").unwrap(), 2020, 2030)
}

fn check_year(year: &str, min: u32, max: u32) -> bool {
    match year.parse::<u32>() {
        Ok(year_value) => year_value >= min && year_value <= max,
        Err(_) => false,
    }
}
