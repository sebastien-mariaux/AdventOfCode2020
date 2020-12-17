use std::collections::BTreeMap;
use std::collections::HashSet;

fn main() {
    let input = String::from(
        "###..#..\n\
         .#######\n\
         #####...\n\
         #..##.#.\n\
         ###..##.\n\
         ##...#..\n\
         ..#...#.\n\
         .#....##",
    );
    println!("And the result is {}", solve_puzzle(input));
}

fn solve_puzzle(data: String) -> u32 {
    let mut area_map: BTreeMap<(i32, i32, i32), char> = BTreeMap::new();
    data.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, state)| {
            area_map.insert((x as i32, y as i32, 0), state);
        })
    });
    let mut min_x: i32 = 0;
    let mut max_x: i32 = data.lines().count() as i32;
    let mut min_y: i32 = 0;
    let mut max_y: i32 = data.lines().next().unwrap().chars().count() as i32;
    let mut min_z: i32 = 0;
    let mut max_z: i32 = 0;

    for _ in 0..6 {
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;
        min_z -= 1;
        max_z += 1;
        area_map = run_cycle(area_map, min_x, max_x, min_y, max_y, min_z, max_z);
    }
    area_map.values().filter(|x| x == &&'#').count() as u32
}

fn run_cycle(
    area_map: BTreeMap<(i32, i32, i32), char>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
) -> BTreeMap<(i32, i32, i32), char> {
    let mut new_area: BTreeMap<(i32, i32, i32), char> = BTreeMap::new();
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            for z in min_z..max_z + 1 {
                let friends = friends((x, y, z));
                let active_friends = friends
                    .iter()
                    .filter(|(xx, yy, zz)|matches!(&area_map.get(&(*xx, *yy, *zz)), Some('#')))
                    .count();
                let state = match area_map.get(&(x, y, z)) {
                    Some(x) => *x,
                    None => '.',
                };
                let next_state = match (state, active_friends) {
                    ('#', 2) => '#',
                    ('#', 3) => '#',
                    ('#', _) => '.',
                    ('.', 3) => '#',
                    _ => '.',
                };

                new_area.insert((x, y, z), next_state);
            }
        }
    }
    new_area
}

fn friends(coord: (i32, i32, i32)) -> HashSet<(i32, i32, i32)> {
    // Because neighbour is too hard to spell xD
    let (x, y, z) = coord;
    let mut friends = HashSet::new();
    for xx in x - 1..x + 2 {
        for yy in y - 1..y + 2 {
            for zz in z - 1..z + 2 {
                if (xx, yy, zz) != coord {
                    friends.insert((xx, yy, zz));
                }
            }
        }
    }
    assert_eq!(3 * 3 * 3 - 1, friends.len());
    friends
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let data = String::from(
            ".#.\n\
             ..#\n\
             ###",
        );
        assert_eq!(112, solve_puzzle(data));
    }

    #[test]
    fn test_input() {
        let input = String::from(
            "###..#..\n\
             .#######\n\
             #####...\n\
             #..##.#.\n\
             ###..##.\n\
             ##...#..\n\
             ..#...#.\n\
             .#....##",
        );
        assert_eq!(207, solve_puzzle(input));
    }
}
