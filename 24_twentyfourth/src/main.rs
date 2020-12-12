use regex::Regex;
use std::fs;

fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

struct Boat {
    waypoint_x: i32,
    waypoint_y: i32,
    x: i32,
    y: i32,
}

impl Boat {
    fn distance(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn new() -> Boat {
        Boat {
            waypoint_x: 10,
            waypoint_y: 1,
            x: 0,
            y: 0,
        }
    }

    fn move_toward(mut self, value: u32) -> Boat {
        self.x += self.waypoint_x * value as i32;
        self.y += self.waypoint_y * value as i32;
        self
    }

    fn move_waypoint(mut self, direction: char, value: u32) -> Boat {
        match direction {
            'N' => self.waypoint_y += value as i32,
            'S' => self.waypoint_y -= value as i32,
            'E' => self.waypoint_x += value as i32,
            'W' => self.waypoint_x -= value as i32,
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
            let keep = self.waypoint_x;
            self.waypoint_x = self.waypoint_y;
            self.waypoint_y = -keep;
        }
        self
    }

    fn turn_left(mut self, count: u32) -> Boat {
        for _ in 0..count {
            let keep = self.waypoint_x;
            self.waypoint_x = -self.waypoint_y;
            self.waypoint_y = keep;
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
        println!("Instruction {}, value {}", instruction, value);

        boat = match instruction {
            'N' => boat.move_waypoint('N', value),
            'S' => boat.move_waypoint('S', value),
            'E' => boat.move_waypoint('E', value),
            'W' => boat.move_waypoint('W', value),
            'L' => boat.turn('L', value),
            'R' => boat.turn('R', value),
            'F' => boat.move_toward(value),
            _ => boat,
        };
        println!("x {}, y {}", boat.x, boat.y);
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
        assert_eq!(286, solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(20873, solve_puzzle("input"));
    }
}
