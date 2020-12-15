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

#[cfg(test)]
mod test {
    use super::*;

    fn test_valid_year(name: &str, year: &str) -> bool{
        let mut passport = HashMap::new();
        passport.insert(String::from(name), String::from(year));
        valid_eyr(&passport)
    }

    #[test]
    fn test_valid_eyr() {
        assert!(test_valid_year("eyr", "2025"));
    }

    #[test]
    fn test_valid_eyr_min_year() {
        assert!(test_valid_year("eyr", "2020"));
    }

    #[test]
    fn test_valid_eyr_max_year() {
        assert!(test_valid_year("eyr", "2030"));
    }

    #[test]
    fn test_invalid_eyr_year_alphanum() {
        assert!(!test_valid_year("eyr", "abcd"));
    }

    #[test]
    fn test_invalid_eyr_year_under_limit() {
        assert!(!test_valid_year("eyr", "2000"))

    }

    #[test]
    fn test_invalid_eyr_year_over_limit() {
        assert!(!test_valid_year("eyr", "2031"))
    }

    fn test_valid_height(name: &str, height: &str) -> bool {
        let mut passport = HashMap::new();
        passport.insert(String::from(name), String::from(height));
        valid_height(&passport)
    }

    #[test]
    fn test_valid_height_inches() {
        assert!(test_valid_height("hgt", "60in"));
    }

    #[test]
    fn test_valid_height_cm() {
        assert!(test_valid_height("hgt", "190cm"));
    }

    #[test]
    fn test_invalid_height_inches() {
        assert!(!test_valid_height("hgt", "190in"));
    }

    #[test]
    fn test_invalid_height_cm() {
        assert!(!test_valid_height("hgt", "60cm"));
    }
    
    #[test]
    fn test_invalid_height_wrong_unit() {
        assert!(!test_valid_height("hgt", "175mm"));
    }

    #[test]
    fn test_invalid_height_no_unit() {
        assert!(!test_valid_height("hgt", "175"));
    }

    #[test]
    fn test_invalid_height_no_number() {
        assert!(!test_valid_height("hgt", "aaacm"));
    }

    fn test_valid_pid(name: &str, pid: &str) -> bool {
        let mut passport = HashMap::new();
        passport.insert(String::from(name), String::from(pid));
        valid_pid(&passport)
    }

    #[test]
    fn test_valid_pid_starting_zeros() {
        assert!(test_valid_pid("pid", "000456789"));
    }

    #[test]
    fn test_invalid_pid() {
        assert!(!test_valid_pid("pid", "0123456789"));
    }

    #[test]
    fn test_a_valid_passport_with_cid() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f cid:88";
        assert!(is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_a_valid_passport_without_cid() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert!(is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_invalid_passport_missing_pid() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert!(!is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_invalid_passport_missing_hgt() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert!(!is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_invalid_passport_missing_ecl() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert!(!is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_a_valid_passport_without_iyr() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in ecl:grn eyr:2030 byr:1980 hcl:#623a2f";
        assert!(!is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_invalid_passport_missing_eyr() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 byr:1980 hcl:#623a2f";
        assert!(!is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_invalid_passport_missing_byr() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 hcl:#623a2f";
        assert!(!is_valid(&passport, &required_fields));
    }

    #[test]
    fn test_invalid_passport_missing_hcl() {
        let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 byr:1980";
        assert!(!is_valid(&passport, &required_fields));
    }
}
