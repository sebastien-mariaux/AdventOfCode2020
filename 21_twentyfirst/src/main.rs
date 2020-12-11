use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u32 {
    let mut layout = parse_layout(read_data(file_name));
    loop {
        let new_layout = run_round(&layout);
        if new_layout == layout {
            break;
        };
        layout = new_layout;
    }
    layout.iter().flatten().filter(|x| x == &&'#').count() as u32
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

fn run_round(layout: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![];
    for (row_idx, row) in layout.iter().enumerate() {
        let mut new_row: Vec<char> = vec![];
        for (col_idx, seat) in row.iter().enumerate() {
            let seat_state = next_state(seat, layout, row_idx, col_idx);
            new_row.push(seat_state);
        }
        output.push(new_row);
    }
    output
}

fn next_state(seat: &char, layout: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> char {
    let min_col = match col_idx == 0 {
        true => 0,
        false => col_idx - 1,
    };
    let max_col = match col_idx == layout[0].len() - 1 {
        true => col_idx,
        false => col_idx + 1,
    };
    let min_row = match row_idx == 0 {
        true => 0,
        false => row_idx - 1,
    };
    let max_row = match row_idx == layout.len() - 1 {
        true => row_idx,
        false => row_idx + 1,
    };

    let mut adjacent_occupied: u8 = 0;
    (min_row..max_row + 1).for_each(|x| {
        (min_col..max_col + 1).for_each(|y| {
            if (x != row_idx || y != col_idx) && layout[x][y] == '#' {
                adjacent_occupied += 1;
            }
        })
    });

    match (seat, adjacent_occupied == 0, adjacent_occupied >= 4) {
        ('.', _, _) => '.',
        ('L', true, _) => '#',
        ('#', _, true) => 'L',
        (x, _, _) => *x,
    }
}

fn parse_layout(layout: String) -> Vec<Vec<char>> {
    layout
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(37, solve_puzzle("example_data"));
    }

    #[test]
    fn test_second_iteration() {
        let entry = parse_layout(String::from(
            "#.##.L#.##\n\
        #L###LL.L#\n\
        L.#.#..#..\n\
        #L##.##.L#\n\
        #.##.LL.LL\n\
        #.###L#.##\n\
        ..#.#.....\n\
        #L######L#\n\
        #.LL###L.L\n\
        #.#L###.##",
        ));
        let output = parse_layout(String::from(
            "#.#L.L#.##\n\
        #LLL#LL.L#\n\
        L.L.L..#..\n\
        #LLL.##.L#\n\
        #.LL.LL.LL\n\
        #.LL#L#.##\n\
        ..L.L.....\n\
        #L#LLLL#L#\n\
        #.LLLLLL.L\n\
        #.#L#L#.##",
        ));
        assert_eq!(output, run_round(&entry));
    }

    #[test]
    fn test_adjacent_occupied_full() {
        let layout = parse_layout(String::from(
            "###\n\
            ###\n\
            ###",
        ));
        assert_eq!('L', next_state(&'#', &layout, 1, 1))
    }

    #[test]
    fn test_adjacent_occupied_empty() {
        let layout = parse_layout(String::from(
            "...\n\
            .#.\n\
            ...",
        ));
        assert_eq!('#', next_state(&'#', &layout, 1, 1))
    }

    #[test]
    fn test_adjacent_occupied_four() {
        let layout = parse_layout(String::from(
            ".##\n\
            .##\n\
            #..",
        ));
        assert_eq!('L', next_state(&'#', &layout, 1, 1))
    }
}
