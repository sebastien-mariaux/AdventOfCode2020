use std::fs;

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
            None
        } else {
            self.col_position += right;
            if self.col_position >= self.map_width {
                self.col_position -= self.map_width;
            }
            Some(self)
        }
    }

    fn has_tree(&self) -> bool {
        self.field_map[self.row_position as usize][self.col_position as usize] == '#'
    }
}

fn main() {
    let mut area = Area::new(read_lines());
    let mut trees_count = 0;
    loop {
        area = match area.slide(1, 3) {
            None => break,
            Some(area) => {
                if area.has_tree() {
                    trees_count += 1
                };
                println!("So far we-ve hit {} trees", trees_count);
                area
            }
        };
    }
    println!("We encountered {} trees on the way, ouch!", trees_count);
}

fn read_lines() -> Vec<Vec<char>> {
    fs::read_to_string("input")
        .expect("Error")
        .lines()
        .map(|x| x.to_string().chars().collect::<Vec<char>>())
        .collect()
}
