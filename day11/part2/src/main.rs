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
    let visible_occupied = count_visible_occupied(layout, row_idx, col_idx);

    match (seat, visible_occupied == 0, visible_occupied >= 5) {
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


fn count_visible_occupied(layout: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u8 {
    let layout_cols = layout[0].len();
    let layout_rows = layout.len();

    let mut visible_occupied: u8 = 0;
    // above
    let mut i = 1;
    while row_idx >= i {
        match layout[row_idx - i][col_idx] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }
    // below
    i = 1;
    while row_idx + i < layout_rows {
        match layout[row_idx + i][col_idx] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }
    // right
    i = 1;
    while col_idx + i < layout_cols {
        match layout[row_idx][col_idx + i] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }
    // left
    i = 1;
    while col_idx >= i {
        match layout[row_idx][col_idx - i] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }

    // top left
    i = 1;
    while col_idx >= i && row_idx >= i {
        match layout[row_idx - i][col_idx - i] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }

    // top right
    i = 1;
    while col_idx + i < layout_cols && row_idx >= i {
        match layout[row_idx - i][col_idx + i] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }
    // bottom right
    i = 1;
    while col_idx + i < layout_cols && row_idx + i < layout_rows {
        match layout[row_idx + i][col_idx + i] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }

    // bottom left
    i = 1;
    while col_idx >= i && row_idx + i < layout_rows {
        match layout[row_idx + i][col_idx - i] {
            '#' => {
                visible_occupied += 1;
                break;
            }
            'L' => break,
            _ => (),
        }
        i += 1;
    }

    visible_occupied
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(26, solve_puzzle("example_data"));
    }

    #[test]
    fn test_second_iteration() {
        let entry = parse_layout(String::from(
            "#.L#.L#.L#\n\
            #LLLLLL.LL\n\
            L.L.L..#..\n\
            ##LL.LL.L#\n\
            L.LL.LL.L#\n\
            #.LLLLL.LL\n\
            ..L.L.....\n\
            LLLLLLLLL#\n\
            #.LLLLL#.L\n\
            #.L#LL#.L#",
        ));
        let output = parse_layout(String::from(
            "#.L#.L#.L#\n\
             #LLLLLL.LL\n\
             L.L.L..#..\n\
             ##L#.#L.L#\n\
             L.L#.#L.L#\n\
             #.L####.LL\n\
             ..#.#.....\n\
             LLL###LLL#\n\
             #.LLLLL#.L\n\
             #.L#LL#.L#",
        ));
        assert_eq!(output, run_round(&entry));
    }

    #[test]
    fn last_iteration() {
        let entry = parse_layout(String::from(
            "#.L#.L#.L#\n\
            #LLLLLL.LL\n\
            L.L.L..#..\n\
            ##L#.#L.L#\n\
            L.L#.#L.L#\n\
            #.L####.LL\n\
            ..#.#.....\n\
            LLL###LLL#\n\
            #.LLLLL#.L\n\
            #.L#LL#.L#",
        ));
        let output = parse_layout(String::from(
            "#.L#.L#.L#\n\
            #LLLLLL.LL\n\
            L.L.L..#..\n\
            ##L#.#L.L#\n\
            L.L#.LL.L#\n\
            #.LLLL#.LL\n\
            ..#.L.....\n\
            LLL###LLL#\n\
            #.LLLLL#.L\n\
            #.L#LL#.L#",
        ));
        assert_eq!(output, run_round(&entry));
    }

    #[test]
    fn test_see_eight_occupied() {
        let layout = parse_layout(String::from(
            ".......#.\n\
            ...#.....\n\
            .#.......\n\
            .........\n\
            ..#L....#\n\
            ....#....\n\
            .........\n\
            #........\n\
            ...#.....",
        ));
        assert_eq!('L', next_state(&'L', &layout, 4, 3))
    }

    #[test]
    fn see_no_occupied_seat() {
        let layout = parse_layout(String::from(
            ".##.##.\n\
            #.#.#.#\n\
            ##...##\n\
            ...L...\n\
            ##...##\n\
            #.#.#.#\n\
            .##.##.",
        ));
        assert_eq!(0, count_visible_occupied(&layout, 3, 3))
    }

    #[test]
    fn no_visible_occupied_right() {
        let entry = parse_layout(String::from("L....L.#"));
        let result = count_visible_occupied(&entry, 0, 0);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_occupied_right() {
        let entry = parse_layout(String::from("L......#"));
        let result = count_visible_occupied(&entry, 0, 0);
        assert_eq!(1, result);
    }

    #[test]
    fn no_visible_occupied_left() {
        let entry = parse_layout(String::from("#..L...L."));
        let result = count_visible_occupied(&entry, 0, 7);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_occupied_left() {
        let entry = parse_layout(String::from("#......L."));
        let result = count_visible_occupied(&entry, 0, 7);
        assert_eq!(1, result);
    }

    #[test]
    fn visible_occupied_left_and_right() {
        let entry = parse_layout(String::from("L..#.#.L...#"));
        let result = count_visible_occupied(&entry, 0, 7);
        assert_eq!(2, result);
    }

    #[test]
    fn visible_occupied_below() {
        let entry = parse_layout(String::from(
            "L\n\
            .\n\
            .\n\
            #\n\
            #",
        ));
        let result = count_visible_occupied(&entry, 0, 0);
        assert_eq!(1, result);
    }

    #[test]
    fn no_visible_occupied_below() {
        let entry = parse_layout(String::from(
            "L\n\
             L\n\
             .\n\
             #\n\
             #",
        ));
        let result = count_visible_occupied(&entry, 0, 0);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_occupied_above() {
        let entry = parse_layout(String::from(
            "#\n\
            #\n\
            .\n\
            L\n\
            .",
        ));
        let result = count_visible_occupied(&entry, 3, 0);
        assert_eq!(1, result);
    }

    #[test]
    fn no_visible_occupied_above() {
        let entry = parse_layout(String::from(
            "#\n\
            #\n\
            L\n\
            L\n\
            .",
        ));
        let result = count_visible_occupied(&entry, 3, 0);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_occupied_above_and_below() {
        let entry = parse_layout(String::from(
            "#\n\
            #\n\
            .\n\
            L\n\
            .\n\
            #",
        ));
        let result = count_visible_occupied(&entry, 3, 0);
        assert_eq!(2, result);
    }

    #[test]
    fn visible_occupied_top_right() {
        let entry = parse_layout(String::from(
            ".....#\n\
            ......\n\
            ...L..\n\
            ......\n\
            ......",
        ));
        let result = count_visible_occupied(&entry, 2, 3);
        assert_eq!(1, result);
    }

    #[test]
    fn no_visible_occupied_top_right() {
        let entry = parse_layout(String::from(
            ".....#\n\
            ....L.\n\
            ...L..\n\
            ......\n\
            ......",
        ));
        let result = count_visible_occupied(&entry, 2, 3);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_occupied_bottom_left() {
        let entry = parse_layout(String::from(
            "......\n\
            ......\n\
            ...L..\n\
            .....\n\
            .#....",
        ));
        let result = count_visible_occupied(&entry, 2, 3);
        assert_eq!(1, result);
    }

    #[test]
    fn no_visible_occupied_bottom_left() {
        let entry = parse_layout(String::from(
            "......\n\
            ......\n\
            ...L..\n\
            ..L..\n\
            .#....",
        ));
        let result = count_visible_occupied(&entry, 2, 3);
        assert_eq!(0, result);
    }

    #[test]
    fn no_visible_occupied_top_left() {
        let entry = parse_layout(String::from(
            "L#L...\n\
             ..L...\n\
             ...L..\n\
             .....\n\
             ......",
        ));
        let result = count_visible_occupied(&entry, 2, 3);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_occupied_top_left() {
        let entry = parse_layout(String::from(
            "L#L...\n\
             ......\n\
             ...L..\n\
             .....\n\
             ......",
        ));
        let result = count_visible_occupied(&entry, 2, 3);
        assert_eq!(1, result);
    }

    #[test]
    fn visible_occupied_bottom_right() {
        let entry = parse_layout(String::from(
            "......\n\
            ......\n\
            .L....\n\
            .....\n\
            ...#..",
        ));
        let result = count_visible_occupied(&entry, 2, 1);
        assert_eq!(1, result);
    }

    #[test]
    fn no_visible_occupied_bottom_right() {
        let entry = parse_layout(String::from(
            "......\n\
             ......\n\
             .L....\n\
             ..L..\n\
             ...#..",
        ));
        let result = count_visible_occupied(&entry, 2, 1);
        assert_eq!(0, result);
    }

    #[test]
    fn visible_nothing_nowhere() {
        let entry = parse_layout(String::from(
            ".....\n\
            .....\n\
            ..#..\n\
            .....\n\
            .....",
        ));
        let result = count_visible_occupied(&entry, 2, 2);
        assert_eq!(0, result);
    }

    #[test]
    fn count_visible_for() {
        let entry = parse_layout(String::from(
            "#.L#.##.L#\n\
             #L#####.LL\n\
             L.#.#..#..\n\
             ##L#.##.##\n\
             #.##.#L.##\n\
             #.#####.#L\n\
             ..#.#.....\n\
             LLL####LL#\n\
             #.L#####.L\n\
             #.L####.L#",
        ));
        let result = count_visible_occupied(&entry, 0, 0);
        assert_eq!(1, result);
    }

    #[test]
    fn eight_visible() {
        let entry = parse_layout(String::from(
            ".......#.\n\
             ...#.....\n\
             .#.......\n\
             .........\n\
             ..#L....#\n\
             ....#....\n\
             .........\n\
             #........\n\
             ...#.....",
        ));
        let result = count_visible_occupied(&entry, 4, 3);
        assert_eq!(8, result);
    }

    #[test]
    fn eight_adjacent_visible() {
        let entry = parse_layout(String::from(
            ".....\n\
             .###.\n\
             .#L#.\n\
             .###.\n\
             .....",
        ));
        let result = count_visible_occupied(&entry, 2, 2);
        assert_eq!(8, result);
    }

    #[test]
    fn test_seat_hidden_by_empty_seat() {
        let entry = parse_layout(String::from(
            ".............\n\
             .L.L.#.#.#.#.\n\
             .............",
        ));
        let result = count_visible_occupied(&entry, 1, 1);
        assert_eq!(0, result);
    }
}
