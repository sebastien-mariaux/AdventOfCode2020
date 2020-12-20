use std::fs;
use std::collections::HashSet;
// use std::collections::BTreeMap;

struct Tile<'a> {
    number: usize,
    map: Vec<Vec<char>>,
    variant: HashSet<Vec<Vec<char>>>,
    right: Option<&'a Tile<'a>>,
    left: Option<&'a Tile<'a>>,
    top: Option<&'a Tile<'a>>,
    bottom: Option<&'a Tile<'a>>,
}

impl<'a> Clone for Tile<'a> {
    fn clone(&self) -> Tile<'a> {
        Tile {
            number: self.number,
            map: self.map.iter().map(|row| row.iter().map(|x| *x).collect()).collect(),
            variant: self.variant.iter().map(|row| row.iter().map(|subrow| subrow.iter().map(|x| *x).collect()).collect()).collect(),
            right: self.right,
            left: self.left,
            top: self.top,
            bottom: self.bottom,
        }
    }
}

impl<'a> Tile<'a> {
    fn new(data: &str) -> Tile {
        let mut iter_data = data.lines();
        let number = iter_data.next().unwrap().replace("Tile ", "").replace(":", "").parse::<usize>().unwrap();
        let map = iter_data.map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();
        let variant = compute_variant(&map);
        Tile {number, map, variant, right: None, left: None, top: None, bottom: None}
    }
}

fn compute_variant(map: &Vec<Vec<char>>) -> HashSet<Vec<Vec<char>>> {
    let mut variant = HashSet::new();
    variant.insert(map.clone());
    let mut new_map = map.clone();
    for _ in 0..3 {
        new_map = flip_right(&new_map);
        variant.insert(new_map.clone());
    };
    new_map = flip_flap(&map);
    variant.insert(new_map.clone());
    for _ in 0..3 {
        new_map = flip_right(&new_map);
        variant.insert(new_map.clone());
    };
    assert_eq!(8, variant.len());
    variant
}

fn flip_right(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for j in 0..map[0].len() {
        let mut new_row = Vec::new();
        for i in (0..map.len()).rev() {
            new_row.push(map[i][j]);
        }
        new_map.push(new_row);
    };
    new_map
}

fn flip_flap(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();
    for i in 0..map.len() {
        let mut new_row = Vec::new();
        for j in (0..map.len()).rev() {
            new_row.push(map[i][j]);
        }
        new_map.push(new_row);
    };
    new_map
}



fn main() {
    let result = solve_puzzle("input");
    println!("And the result is {}", result);
}

fn solve_puzzle(file_name: &str) -> u64 {
    let data = read_data(file_name);
    let mut tiles = Vec::new();
    data.split("\n\n").for_each(|x|{
        tiles.push(Tile::new(x));
    });
    for i in 0..tiles.len() {
        let mut tile = tiles[i].clone();
        for j in 0..tiles.len() {
            let other_tile = &tiles[j];
            if i != j {}
            // tile.right = Some(other_tile);
        }
    };
    0
}

fn read_data(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Error")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(20899048083289, solve_puzzle("example_data"));
    }

    #[test]
    fn test_input() {
        assert_eq!(1, solve_puzzle("input"));
    }

    #[test]
    fn test_flip() {
        let data = "###..##..#\n\
                        ....##.###\n\
                        ###..##..#\n\
                        ..#..#.#.#\n\
                        #.........\n\
                        ##.#...#.#\n\
                        .........#\n\
                        ....#..#.#\n\
                        #.........\n\
                        ###.#...#.";
        let map = data.lines().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

        let output_data = "##..##.#.#\n\
                                #...#..#.#\n\
                                #.....##.#\n\
                                ....#.....\n\
                                #.#.....#.\n\
                                ......####\n\
                                .......#.#\n\
                                ..#.#.#.#.\n\
                                #.......#.\n\
                                ..###.####";
        let output = output_data.lines().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

        assert_eq!(output, flip_right(&map));
    }

    #[test]
    fn test_flip_flap() {
        let data = "###..##..#\n\
                        ....##.###\n\
                        ###..##..#\n\
                        ..#..#.#.#\n\
                        #.........\n\
                        ##.#...#.#\n\
                        .........#\n\
                        ....#..#.#\n\
                        #.........\n\
                        ###.#...#.";
        let map = data.lines().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

        let output_data = "#..##..###\n\
                                ###.##....\n\
                                #..##..###\n\
                                #.#.#..#..\n\
                                .........#\n\
                                #.#...#.##\n\
                                #.........\n\
                                #.#..#....\n\
                                .........#\n\
                                .#...#.###";
        let output = output_data.lines().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>();

        assert_eq!(output, flip_flap(&map));
    }

}