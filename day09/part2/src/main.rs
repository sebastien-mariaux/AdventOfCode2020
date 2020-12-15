use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let numbers = data
        .lines()
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let (_, result) = &numbers
        .iter()
        .enumerate()
        .find(|(idx, x)| {
            let slice = &numbers[0..*idx];
            idx > &24
                && !slice
                    .iter()
                    .any(|y| (y <= x && numbers[0..*idx].contains(&x.wrapping_sub(*y))))
        })
        .unwrap();

    numbers
        .iter()
        .enumerate()
        .filter(|(_, x)| x != result)
        .find(|(index, _)| {
            let mut set: Vec<u128> = vec![];
            let mut acc: u128 = 0;
            let mut finish = false;
            let mut iter = numbers[0..*index].iter().rev();
            while let Some(value) = iter.next() {
                set.push(*value);
                acc += value;
                match (&&acc > result, &&acc == result) {
                    (true, _) => break,
                    (_, true) => {
                        finish = true;
                        handle_set(set.clone(), **result);
                    }
                    _ => (),
                }
            }
            finish
        });
}

fn handle_set(set: Vec<u128>, result: u128) {
    println!("{:?}", set);
    let max = set
        .iter()
        .fold(0, |acc, x| if &acc >= x { acc } else { *x });
    let min = set
        .iter()
        .fold(result, |acc, x| if &acc <= x { acc } else { *x });
    println!("And the result is {}", max + min);
}
