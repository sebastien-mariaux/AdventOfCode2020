use std::fs;

fn main() {
    let mut numbers = read_numbers();
    numbers.sort();

    loop {
        let number = numbers.pop().unwrap();
        let target = 2020 - number;

        if (numbers.iter().find(|&&x| x == target)).is_some() {
            println!(
                "And the result is {} which is the product of {} and {}",
                target * number,
                target,
                number
            );
            break;
        }
    }
}

fn read_numbers() -> Vec<u32> {
    let filename = "input";
    let data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    data.lines()
        .map(|x| match x.parse::<u32>() {
            Ok(x) => x,
            Err(_) => 0,
        })
        .collect::<Vec<u32>>()
}
