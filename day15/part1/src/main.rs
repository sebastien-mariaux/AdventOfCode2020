use std::collections::HashMap;

fn main() {
    let result = solve_puzzle("0,1,4,13,15,12,16");
    println!("And the result is {}", result);
}

fn solve_puzzle(entry: &str) -> u32 {
    let numbers_vec = entry
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut last_number = *numbers_vec.iter().rev().nth(0).unwrap();

    let mut numbers = numbers_vec.iter()
        .enumerate()
        .map( |(idx, x)| (*x, (idx as u32, idx as u32)))
        .collect::<HashMap<u32, (u32, u32)>>();

    for i in numbers.len()..2020 {
        let new_number = match numbers.get(&last_number) {
            Some((a, b)) => a - b,
            None => panic!("Not possible"),
        };
        println!("New number {}", new_number);
        match numbers.get_mut(&new_number) {
            Some((a, b)) => {
                *b = a.clone();
                *a = i as u32;
            }
            None => {
                numbers.insert(new_number, (i as u32, i as u32));
            }
        }
        println!("{:?}", numbers);
        last_number = new_number;
    }
    last_number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(436, solve_puzzle("0,3,6"));
        assert_eq!(1, solve_puzzle("1,3,2"));
        assert_eq!(10, solve_puzzle("2,1,3"));
        assert_eq!(27, solve_puzzle("1,2,3"));
        assert_eq!(78, solve_puzzle("2,3,1"));
        assert_eq!(438, solve_puzzle("3,2,1"));
        assert_eq!(1836, solve_puzzle("3,1,2"));
    }
}
