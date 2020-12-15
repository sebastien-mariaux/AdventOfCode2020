use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let count: u32 = data.split("\n\n").map(|group| count_for_group(group)).sum();
    println!("oy, oy, oy, the result is {}", count);
}

fn count_for_group(answers: &str) -> u32 {
    let mut cleaned_answer = answers.replace("\n", "").chars().collect::<Vec<char>>();
    cleaned_answer.sort_unstable();
    cleaned_answer.dedup();
    cleaned_answer.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_for_group() {
        assert_eq!(3, count_for_group("abc"));
        assert_eq!(3, count_for_group("a\nb\nc"));
        assert_eq!(3, count_for_group("ab\nac"));
        assert_eq!(1, count_for_group("a\na\na\na"));
    }
}
