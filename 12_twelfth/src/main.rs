use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let count: u32 = data.split("\n\n").map(|group| count_for_group(group)).sum();
    println!("oy, oy, oy, the result is {}", count);
}

fn count_for_group(answers: &str) -> u32 {
    let members = answers.split("\n").collect::<Vec<&str>>();
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    alpha
        .chars()
        .filter(|letter| members.iter().all(|x| x.contains(*letter) ))
        .collect::<String>()
        .len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_for_group() {
        assert_eq!(3, count_for_group("abc"));
        assert_eq!(0, count_for_group("a\nb\nc"));
        assert_eq!(1, count_for_group("ab\nac"));
        assert_eq!(1, count_for_group("a\na\na\na"));
    }
}
