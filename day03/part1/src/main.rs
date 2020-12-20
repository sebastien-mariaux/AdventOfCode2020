use std::fs;


fn main() {
    let result = solve_puzzle();
    println!("We encountered {} trees on the way, ouch!", result);
}

fn solve_puzzle() -> usize {
    let mut area = Area::new(read_lines());
    let mut trees_count = 0;
    while let Some(new_area) = area.slide(1, 3) {
        if new_area.has_tree() {
            trees_count += 1
        };
        area = new_area
    };
    trees_count
}

struct Area {
    field_map: Vec<Vec<char>>,
    map_width: u32,
    map_height: u32,
    row_position: u32,
    col_position: u32,
}

impl Area {
    fn new(field_map: Vec<Vec<char>>) -> Area {
        let width = field_map[0].len();
        let height = field_map.len();
        Area {
            field_map,
            map_width: width as u32,
            map_height: height as u32,
            row_position: 0,
            col_position: 0,
        }
    }

    fn slide(mut self, down: u32, right: u32) -> Option<Area> {
        self.row_position += down;
        if self.row_position >= self.map_height {
            return None;
        } 

        self.col_position += right;
        if self.col_position >= self.map_width {
            self.col_position -= self.map_width;
        }
        Some(self)
    }

    fn has_tree(&self) -> bool {
        self.field_map[self.row_position as usize][self.col_position as usize] == '#'
    }
}

fn read_lines() -> Vec<Vec<char>> {
    fs::read_to_string("input")
        .expect("Error")
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(156, solve_puzzle());
    }
}