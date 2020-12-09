use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let numbers = data.lines().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();

    let result = &numbers.iter().enumerate().find(|(idx, x)| {
        let slice = &numbers[0..*idx];
        idx > &24 && !slice.iter().any(|y| (y <= x && numbers[0..*idx].contains(&x.wrapping_sub(*y)) ) )
    } ).unwrap();

    println!("{}", result.1);
}
