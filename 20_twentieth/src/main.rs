use std::fs;
use std::collections::BTreeMap;

fn main() {
    // _very_inefficient_solution();
    much_better_although_probably_not_the_best_solution();
}

fn much_better_although_probably_not_the_best_solution() {
    let data = fs::read_to_string("input").expect("Error");
    let mut numbers: Vec<u32> = data.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.push(0);
    numbers.sort_unstable();
    numbers.reverse();
    let mut map: BTreeMap<u32, u128> = BTreeMap::new();
    numbers.iter().for_each(|x| {
        let number_of_paths = count_path_from(*x, &numbers, &map);
        map.insert(*x, number_of_paths);
    });
    println!("Result {}", map.get(&0).unwrap());
}

fn count_path_from(value: u32, numbers: &[u32], map: &BTreeMap<u32, u128>) -> u128 {
    if numbers.iter().position(|x| x == &value).unwrap() == 0 {
        return 1;
    }
    let mut counter = 0;
    [value + 1, value + 2, value + 3].iter().for_each(|x| {
        if numbers.iter().find(|y| y == &x).is_some() {
            counter += map.get(x).unwrap();
        }
    });

    counter
}

fn _very_inefficient_solution() {
    // This works on examples but takes WAY too long to solve the problem
    let data = fs::read_to_string("input").expect("Error");
    let mut numbers: Vec<u32> = data.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.push(0);
    numbers.sort_unstable();
    let result = _count_path_from(numbers[0], 0, &numbers);
    println!("Result {}", result);
}

fn _count_path_from(current_value: u32, current_index: usize, numbers: &[u32]) -> u32 {
    let mut counter:u32 = 0;
    if current_index == numbers.len() - 1 {
        return counter + 1;
    }
    let mut i = 1;
    loop {
        let next_index = current_index + i;
        if next_index >= numbers.len()    {
            break;
        }
        match numbers[next_index] - current_value <= 3 {
            true => {
                counter += _count_path_from(numbers[next_index], next_index, &numbers);
            },
            false => break,
        };
        i += 1;
    }
    counter
}
