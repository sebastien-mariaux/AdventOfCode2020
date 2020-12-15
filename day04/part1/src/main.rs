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

fn is_valid(passport: &str, required_fields: &Vec<&str>) -> bool {
    if required_fields
        .iter()
        .any(|field| !passport.contains(field))
    {
        return false;
    };
    true
}
