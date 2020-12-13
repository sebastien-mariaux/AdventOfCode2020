use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let file = read_data(file_name);
    let mut data = file.lines();
    let earliest = data.next().unwrap().parse::<u32>().unwrap();
    let bus_ids = data
        .next()
        .unwrap()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut counter = earliest;
    let (bus, next_time) = loop {
        if let Some(bus) = bus_ids.iter().find(|bus| counter % **bus == 0) {
            break (*bus, counter);
        }
        counter += 1;
    };
    (next_time - earliest) * bus
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_example_data() {
        assert_eq!(295, solve_puzzle("example_data"))
    }

    #[test]
    fn solve_input() {
        assert_eq!(138, solve_puzzle("input"))
    }
}
