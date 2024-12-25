use glam::{BVec2, IVec2, UVec2};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
fn main() {
    let inputs = include_str!("input.txt");
    let map = parse_grid(inputs);
    let grid_size = get_grid_size(inputs);
    let antinodes = find_antinodes(&map, grid_size);
    println!("{:?}", antinodes.len());
}

fn find_antinodes(map: &HashMap<char, Vec<UVec2>>, grid_size: UVec2) -> HashSet<UVec2> {
    let mut antinodes: HashSet<UVec2> = HashSet::new();
    for node in map {
        for antenna in node.1 {
            for antenna_pair in node.1 {
                let diff = antenna.as_ivec2() - antenna_pair.as_ivec2();
                if diff.cmpeq(IVec2::ZERO) == BVec2::TRUE {
                    continue;
                }
                let new_antinode =
                    IVec2::new(diff.x + antenna.as_ivec2().x, diff.y + antenna.as_ivec2().y);
                if new_antinode.x < 0 || new_antinode.y < 0 {
                    continue;
                }

                if new_antinode.x >= grid_size.as_ivec2().x
                    || new_antinode.y >= grid_size.as_ivec2().y
                {
                    continue;
                }
                antinodes.insert(new_antinode.as_uvec2());
            }
        }
    }
    antinodes
}
fn get_grid_size(input: &str) -> UVec2 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    UVec2::new(rows.try_into().unwrap(), cols.try_into().unwrap())
}

fn parse_grid(input: &str) -> HashMap<char, Vec<UVec2>> {
    let mut map: HashMap<char, Vec<UVec2>> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (pos, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            if let Entry::Vacant(e) = map.entry(char) {
                e.insert(vec![UVec2::new(
                    pos.try_into().unwrap(),
                    row.try_into().unwrap(),
                )]);
            } else {
                map.get_mut(&char)
                    .unwrap()
                    .push(UVec2::new(pos.try_into().unwrap(), row.try_into().unwrap()));
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_TEST_INPUT: &str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

    const SIMPLE_TEST_INPUT_3_ANTENNAS: &str = "..........
    ..........
    ..........
    ....a.....
........a.
.....a....
..........
..........
..........
..........";

    const SIMPLE_TEST_INPUT_2_FREQUENCIES: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

    const COMPLEX_EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn parse_simple_input() {
        let map = parse_grid(SIMPLE_TEST_INPUT);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&'a').unwrap().len(), 2);
        assert!(map.get(&'a').unwrap().contains(&UVec2::new(4, 3)));
        assert!(map.get(&'a').unwrap().contains(&UVec2::new(5, 5)));
    }
    #[test]
    fn find_simple_anitnodes() {
        let map = parse_grid(SIMPLE_TEST_INPUT);
        let grid_size = get_grid_size(SIMPLE_TEST_INPUT);
        let antinodes = find_antinodes(&map, grid_size);

        assert_eq!(antinodes.len(), 2);
        assert!(antinodes.contains(&UVec2::new(3, 1)));
        assert!(antinodes.contains(&UVec2::new(6, 7)));
    }
    #[test]
    fn find_simple_anitnodes_for_3() {
        let map = parse_grid(SIMPLE_TEST_INPUT_3_ANTENNAS);
        let grid_size = get_grid_size(SIMPLE_TEST_INPUT_3_ANTENNAS);
        let antinodes = find_antinodes(&map, grid_size);

        assert_eq!(antinodes.len(), 4);
        assert!(antinodes.contains(&UVec2::new(3, 1)));
        assert!(antinodes.contains(&UVec2::new(6, 7)));
        assert!(antinodes.contains(&UVec2::new(2, 6)));
        assert!(antinodes.contains(&UVec2::new(0, 2)));
    }
    #[test]
    fn find_simple_anitnodes_for_2_frequencies() {
        let map = parse_grid(SIMPLE_TEST_INPUT_2_FREQUENCIES);
        let grid_size = get_grid_size(SIMPLE_TEST_INPUT_2_FREQUENCIES);
        let antinodes = find_antinodes(&map, grid_size);

        assert_eq!(antinodes.len(), 4);
        assert!(antinodes.contains(&UVec2::new(3, 1)));
        assert!(antinodes.contains(&UVec2::new(6, 7)));
        assert!(antinodes.contains(&UVec2::new(2, 6)));
        assert!(antinodes.contains(&UVec2::new(0, 2)));
    }
    #[test]
    fn find_complex_anitnodes() {
        let map = parse_grid(COMPLEX_EXAMPLE);
        let grid_size = get_grid_size(COMPLEX_EXAMPLE);
        let antinodes = find_antinodes(&map, grid_size);

        assert_eq!(antinodes.len(), 14);
        assert!(antinodes.contains(&UVec2::new(6, 0)));
        assert!(antinodes.contains(&UVec2::new(11, 0)));
        assert!(antinodes.contains(&UVec2::new(3, 1)));
        assert!(antinodes.contains(&UVec2::new(4, 2)));
        assert!(antinodes.contains(&UVec2::new(10, 2)));
        assert!(antinodes.contains(&UVec2::new(2, 3)));
        assert!(antinodes.contains(&UVec2::new(9, 4)));
        assert!(antinodes.contains(&UVec2::new(1, 5)));
        assert!(antinodes.contains(&UVec2::new(3, 6)));
        assert!(antinodes.contains(&UVec2::new(0, 7)));
        assert!(antinodes.contains(&UVec2::new(7, 7)));
        assert!(antinodes.contains(&UVec2::new(10, 10)));
        assert!(antinodes.contains(&UVec2::new(10, 11)));
    }
}
