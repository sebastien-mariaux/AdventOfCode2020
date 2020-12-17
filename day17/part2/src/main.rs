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
    let mut area_map: BTreeMap<(i32, i32, i32, i32), char> = BTreeMap::new();
    data.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, state)| {
            area_map.insert((x as i32, y as i32, 0, 0), state);
        })
    });
    let mut min_coord: i32 = 0;
    let mut max_x: i32 = data.lines().count() as i32;
    let mut max_y: i32 = data.lines().next().unwrap().chars().count() as i32;
    let mut max_coord: i32 = 0;

    for _ in 0..6 {
        min_coord -= 1;
        max_x += 1;
        max_y += 1;
        max_coord += 1;
        area_map = run_cycle(
            area_map,
            (min_coord, max_coord, max_x, max_y),
        );
    }
    area_map.values().filter(|x| x == &&'#').count() as u32
}

fn run_cycle(
    area_map: BTreeMap<(i32, i32, i32, i32), char>,
    ranges: (i32, i32, i32, i32),
) -> BTreeMap<(i32, i32, i32, i32), char> {
    let (min_coord, max_coord, max_x, max_y) = ranges;
    let mut new_area: BTreeMap<(i32, i32, i32, i32), char> = BTreeMap::new();
    for x in min_coord..max_x + 1 {
        for y in min_coord..max_y + 1 {
            for z in min_coord..max_coord + 1 {
                for k in min_coord..max_coord +1 {
                    let friends = friends((x, y, z, k));
                    let active_friends = friends
                        .iter()
                        .filter(|(xx, yy, zz, kk)| {
                            matches!(&area_map.get(&(*xx, *yy, *zz, *kk)), Some('#'))
                        })
                        .count();
                    let state = match area_map.get(&(x, y, z, k)) {
                        Some(state) => *state,
                        None => '.',
                    };
                    let next_state = match (state, active_friends) {
                        ('#', 2) => '#',
                        ('#', 3) => '#',
                        ('#', _) => '.',
                        ('.', 3) => '#',
                        _ => '.',
                    };

                    new_area.insert((x, y, z, k), next_state);
                }
            }
        }
    }
    new_area
}

fn friends(coord: (i32, i32, i32, i32)) -> HashSet<(i32, i32, i32, i32)> {
    // Because neighbour is too hard to spell xD
    let (x, y, z, k) = coord;
    let mut friends = HashSet::new();
    for xx in x - 1..x + 2 {
        for yy in y - 1..y + 2 {
            for zz in z - 1..z + 2 {
                for kk in k - 1..k + 2 {
                    if (xx, yy, zz, kk) != coord {
                        friends.insert((xx, yy, zz, kk));
                    }
                }
            }
        }
    }
    assert_eq!(80, friends.len());
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
        assert_eq!(848, solve_puzzle(data));
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
        assert_eq!(2308, solve_puzzle(input));
    }
}
