use std::fs;

fn main() {
    let result = solve_puzzle();
    println!("The result is: {}", result);
}

fn solve_puzzle() -> usize {
    let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let data = fs::read_to_string("input").expect("Error");
    data
        .split("\n\n")
        .map(|x| x.to_string().replace("\n", " "))
        .filter(|passport| is_valid(passport, &required_fields))
        .count()
}

fn is_valid(passport: &str, required_fields: &Vec<&str>) -> bool {
    if required_fields
        .iter()
        .any(|field| !passport.contains(field))
    {
        return false;
    };
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(200, solve_puzzle());
    }
}