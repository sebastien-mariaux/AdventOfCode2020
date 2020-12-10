use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let mut numbers: Vec<u32> = data.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.sort_unstable();
    numbers = numbers
        .iter()
        .enumerate()
        .map(|(idx, x)| x - previous(idx, &numbers))
        .collect();
    let ones = numbers.iter().filter(|x| x == &&1).count();
    let threes = numbers.iter().filter(|x| x == &&3).count() + 1;
    println!("Ones {}", ones);
    println!("Threes {}", threes);
    println!("The result is {}", ones * threes);
}

fn previous(idx: usize, vec: &[u32]) -> u32 {
    match idx {
        0 => 0,
        _ => vec[idx - 1],
    }
}
