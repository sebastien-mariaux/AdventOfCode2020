use std::collections::HashMap;

fn main() {
    let result_part_1 = solve_puzzle("0,1,4,13,15,12,16", 2020);
    let result_part_2 = solve_puzzle("0,1,4,13,15,12,16", 30000000);
    println!("And the result for part 1 is {}", result_part_1);
    println!("And the result for part 2 is {}", result_part_2);
}

fn solve_puzzle(entry: &str, target: usize) -> u32 {
    let numbers_vec = entry
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut last_number = *numbers_vec.iter().rev().nth(0).unwrap();
    let mut numbers = numbers_vec.iter()
        .enumerate()
        .map( |(idx, x)| (*x, (idx as u32, idx as u32)))
        .collect::<HashMap<u32, (u32, u32)>>();

    for i in numbers.len()..target {
        let new_number = match numbers.get(&last_number) {
            Some((a, b)) => a - b,
            None => panic!("Not possible"),
        };
        match numbers.get_mut(&new_number) {
            Some((a, b)) => {
                *b = a.clone();
                *a = i as u32;
            }
            None => {
                numbers.insert(new_number, (i as u32, i as u32));
            }
        }
        last_number = new_number;
    }
    last_number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(1665, solve_puzzle("0,1,4,13,15,12,16", 2020))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(16439, solve_puzzle("0,1,4,13,15,12,16", 30000000))
    }

    #[test]
    fn test_examples() {
        assert_eq!(436, solve_puzzle("0,3,6", 2020));
        assert_eq!(1, solve_puzzle("1,3,2", 2020));
        assert_eq!(10, solve_puzzle("2,1,3", 2020));
        assert_eq!(27, solve_puzzle("1,2,3", 2020));
        assert_eq!(78, solve_puzzle("2,3,1", 2020));
        assert_eq!(438, solve_puzzle("3,2,1", 2020));
        assert_eq!(1836, solve_puzzle("3,1,2", 2020));
    }

    #[test]
    fn test_examples_part_2_one() {
        assert_eq!(175594, solve_puzzle("0,3,6", 30000000));
    }

    #[test]
    fn test_examples_part_2_two() {
        assert_eq!(2578, solve_puzzle("1,3,2", 30000000));
    }

    #[test]
    fn test_examples_part_2_three() {
        assert_eq!(3544142, solve_puzzle("2,1,3", 30000000));
    }

    #[test]
    fn test_examples_part_2_four() {
        assert_eq!(261214, solve_puzzle("1,2,3", 30000000));
    }

    #[test]
    fn test_examples_part_2_five() {
        assert_eq!(6895259, solve_puzzle("2,3,1", 30000000));
    }

    #[test]
    fn test_examples_part_2_six() {
        assert_eq!(18, solve_puzzle("3,2,1", 30000000));
    }

    #[test]
    fn test_examples_part_2_seven() {
        assert_eq!(362, solve_puzzle("3,1,2", 30000000));
    }
}
