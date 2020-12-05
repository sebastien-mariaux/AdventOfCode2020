use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let result = data.lines().map(|x| seat_id(x)).max().unwrap();
    println!("Oh! oh! oh! The result is {}", result);
}

fn seat_id(binary: &str) -> u32 {
    seat_row(&binary) * 8 + seat_col(&binary)
}

fn seat_row(binary: &str) -> u32 {
    let mut range = (0, 127);
    binary.to_string().chars().take(7).for_each(|letter| {
        range = localize_char(letter, range);
    });

    range.0
}

fn seat_col(binary: &str) -> u32 {
    let mut range = (0, 7);
    binary.to_string().chars().skip(7).for_each(|letter| {
        range = localize_char(letter, range);
    });

    range.0
}

fn localize_char(letter: char, range: (u32, u32)) -> (u32, u32) {
    let (min, max) = range;
    let middle = (max + min + 1) / 2;
    match letter {
        'F' | 'L' => (min, middle - 1),
        'B' | 'R' => (middle, max),
        _ => panic!("What on the snowing earth is this character??"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_seat_id() {
        assert_eq!(357, seat_id("FBFBBFFRLR"));
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }

    #[test]
    fn test_decode_row() {
        assert_eq!(44, seat_row("FBFBBFFRLR"));
        assert_eq!(70, seat_row("BFFFBBFRRR"));
        assert_eq!(14, seat_row("FFFBBBFRRR"));
        assert_eq!(102, seat_row("BBFFBBFRLL"));
    }

    #[test]
    fn test_decode_col() {
        assert_eq!(5, seat_col("FBFBBFFRLR"));
        assert_eq!(7, seat_col("BFFFBBFRRR"));
        assert_eq!(7, seat_col("FFFBBBFRRR"));
        assert_eq!(4, seat_col("BBFFBBFRLL"));
    }

    #[test]
    fn test_localize_row_char() {
        assert_eq!((0, 63), localize_char('F', (0, 127)));
        assert_eq!((32, 63), localize_char('B', (0, 63)));
        assert_eq!((32, 47), localize_char('F', (32, 63)));
        assert_eq!((44, 44), localize_char('F', (44, 45)));
    }

    #[test]
    fn test_localize_col_char() {
        assert_eq!((4, 7), localize_char('R', (0, 7)));
        assert_eq!((4, 5), localize_char('L', (4, 7)));
        assert_eq!((5, 5), localize_char('R', (4, 5)));
    }
}
