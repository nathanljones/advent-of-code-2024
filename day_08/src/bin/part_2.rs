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
        if node.1.len() == 1 {
            continue;
        }
        for antenna in node.1 {
            for antenna_pair in node.1 {
                let mut diff = antenna.as_ivec2() - antenna_pair.as_ivec2();
                let orig_diff = antenna.as_ivec2() - antenna_pair.as_ivec2();
                if diff.cmpeq(IVec2::ZERO) == BVec2::TRUE {
                    let new_antinode = antenna;
                    antinodes.insert(*new_antinode);
                    continue;
                }

                let mut left_grid = false;
                while !left_grid {
                    let new_antinode =
                        IVec2::new(diff.x + antenna.as_ivec2().x, diff.y + antenna.as_ivec2().y);
                    if new_antinode.x < 0 || new_antinode.y < 0 {
                        left_grid = true;
                        break;
                    }

                    if new_antinode.x >= grid_size.as_ivec2().x
                        || new_antinode.y >= grid_size.as_ivec2().y
                    {
                        left_grid = true;
                        break;
                    }
                    antinodes.insert(new_antinode.as_uvec2());
                    diff = diff + orig_diff;
                }
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

    const SIMPLE_TEST_INPUT: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
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
        assert_eq!(map.get(&'T').unwrap().len(), 3);
        assert!(map.get(&'T').unwrap().contains(&UVec2::new(0, 0)));
        assert!(map.get(&'T').unwrap().contains(&UVec2::new(3, 1)));
        assert!(map.get(&'T').unwrap().contains(&UVec2::new(1, 2)));
    }
    #[test]
    fn find_antinodes_simple() {
        let map = parse_grid(SIMPLE_TEST_INPUT);
        let grid_size = get_grid_size(SIMPLE_TEST_INPUT);
        let antinodes = find_antinodes(&map, grid_size);
        assert_eq!(antinodes.len(), 9);
        assert!(antinodes.contains(&UVec2::new(5, 0)));
        assert!(antinodes.contains(&UVec2::new(6, 2)));
        assert!(antinodes.contains(&UVec2::new(9, 3)));
        assert!(antinodes.contains(&UVec2::new(2, 4)));
        assert!(antinodes.contains(&UVec2::new(3, 6)));
        assert!(antinodes.contains(&UVec2::new(4, 8)));
        assert!(antinodes.contains(&UVec2::new(0, 0)));
        assert!(antinodes.contains(&UVec2::new(3, 1)));
        assert!(antinodes.contains(&UVec2::new(1, 2)));
    }
    #[test]
    fn find_antinodes_complex() {
        let map = parse_grid(COMPLEX_EXAMPLE);
        let grid_size = get_grid_size(COMPLEX_EXAMPLE);
        let antinodes = find_antinodes(&map, grid_size);
        assert_eq!(antinodes.len(), 34);
        assert!(antinodes.contains(&UVec2::new(0, 0)));
        assert!(antinodes.contains(&UVec2::new(1, 0)));
        assert!(antinodes.contains(&UVec2::new(6, 0)));
        assert!(antinodes.contains(&UVec2::new(11, 0)));
        /*assert!(antinodes.contains(&UVec2::new(3, 6)));
        assert!(antinodes.contains(&UVec2::new(4, 8)));
        assert!(antinodes.contains(&UVec2::new(0, 0)));
        assert!(antinodes.contains(&UVec2::new(3, 1)));
        assert!(antinodes.contains(&UVec2::new(1, 2)));*/
    }
}
