use std::fs;
use std::collections::HashMap;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u128 {
    let file = read_data(file_name);
    solve_from_string(file)
}

fn solve_from_string(file: String) -> u128 {
    let mut data = file.lines();
    let buses = data
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(idx, x)| (x.parse::<u128>().unwrap(), idx as u128))
        .collect::<HashMap<u128, u128>>();

    let mut keys = buses.keys().map(|x| *x).collect::<Vec<u128>>();
    keys.sort_unstable();
    keys.reverse();
    let slowest = keys[0];
    let mut timestamp: u128 = 0;
    let ts = loop {
        let ts_to_check = match &timestamp > buses.get(&slowest).unwrap() {
            true => timestamp - buses.get(&slowest).unwrap(),
            false => timestamp
        };
        match buses.iter().all(|(bus_id, delta)| (ts_to_check + delta) % bus_id == 0) {
            true => break ts_to_check,
            false => timestamp += slowest,
        }
    };
    ts
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_example_data() {
        assert_eq!(1068781, solve_puzzle("example_data"))
    }

    #[test]
    fn solve_example_one() {
        let entry = String::from("123\n17,x,13,19");
        assert_eq!(3417, solve_from_string(entry));
    }

    #[test]
    fn solve_example_two() {
        let entry = String::from("123\n67,7,59,61");
        assert_eq!(754018, solve_from_string(entry));
    }

    #[test]
    fn solve_example_three() {
        let entry = String::from("123\n67,x,7,59,61");
        assert_eq!(779210, solve_from_string(entry));
    }

    #[test]
    fn solve_example_four() {
        let entry = String::from("123\n67,7,x,59,61");
        assert_eq!(1261476, solve_from_string(entry));
    }

    #[test]
    fn solve_example_five() {
        let entry = String::from("123\n1789,37,47,1889");
        assert_eq!(1202161486, solve_from_string(entry));
    }


    #[test]
    #[ignore = "Not ready yet"]
    fn solve_input() {
        assert_eq!(1, solve_puzzle("input"))
    }
}
