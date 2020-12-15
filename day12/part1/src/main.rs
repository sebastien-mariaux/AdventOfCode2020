use regex::Regex;
use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

struct Boat {
    facing: char,
    x: i32,
    y: i32,
}

impl Boat {
    fn distance(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn new() -> Boat {
        Boat {
            facing: 'E',
            x: 0,
            y: 0,
        }
    }

    fn move_toward(mut self, mut direction: char, value: u32) -> Boat {
        if direction == 'F' {
            direction = self.facing;
        }
        match direction {
            'N' => self.y += value as i32,
            'S' => self.y -= value as i32,
            'E' => self.x += value as i32,
            'W' => self.x -= value as i32,
            _ => (),
        }
        self
    }

    fn turn(mut self, direction: char, value: u32) -> Boat {
        self = match direction {
            'R' => self.turn_right(value / 90),
            'L' => self.turn_left(value / 90),
            _ => self,
        };

        self
    }

    fn turn_right(mut self, count: u32) -> Boat {
        for _ in 0..count {
            match self.facing {
                'N' => self.facing = 'E',
                'E' => self.facing = 'S',
                'S' => self.facing = 'W',
                'W' => self.facing = 'N',
                _ => (),
            }
        }
        self
    }

    fn turn_left(mut self, count: u32) -> Boat {
        for _ in 0..count {
            match self.facing {
                'N' => self.facing = 'W',
                'W' => self.facing = 'S',
                'S' => self.facing = 'E',
                'E' => self.facing = 'N',
                _ => (),
            }
        }
        self
    }
}

fn solve_puzzle(file_name: &str) -> u32 {
    let data = read_data(file_name);

    let mut boat = Boat::new();
    let iterator = data.lines().map(|line| {
        let re = Regex::new(r"^([A-Z])(\d*)$").unwrap();
        let caps = re.captures(line).unwrap();
        (
            caps[1].chars().next().unwrap(),
            caps[2].parse::<u32>().unwrap(),
        )
    });
    for (instruction, value) in iterator {
        boat = match instruction {
            'N' => boat.move_toward('N', value),
            'S' => boat.move_toward('S', value),
            'E' => boat.move_toward('E', value),
            'W' => boat.move_toward('W', value),
            'L' => boat.turn('L', value),
            'R' => boat.turn('R', value),
            'F' => boat.move_toward('F', value),
            _ => boat,
        };
    }

    boat.distance()
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(25, solve_puzzle("example_data"));
    }


    #[test]
    fn test_input() {
        assert_eq!(1687, solve_puzzle("input"));
    }
}
