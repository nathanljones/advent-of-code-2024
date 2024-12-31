use day_10::{parse, Direction};
use glam::UVec2;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

fn count_trailheads(map: &HashMap<UVec2, u32>) -> u32 {
    let start_positions: Vec<(UVec2, u32)> =
        map.clone().into_iter().filter(|pos| pos.1 == 0).collect();
    let end_positions: Vec<(UVec2, u32)> =
        map.clone().into_iter().filter(|pos| pos.1 == 9).collect();
    let mut ret: u32 = 0;
    for start_pos in &start_positions {
        ret += &end_positions.iter().fold(0, |acc, pos| {
            acc + count_paths(
                start_pos.0,
                |&p| get_next_nodes(p, map).into_iter(),
                |&p| p == pos.0,
            ) as u32
        });
    }

    ret
}

fn get_next_nodes(point: UVec2, map: &HashMap<UVec2, u32>) -> Vec<UVec2> {
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut ret: Vec<UVec2> = Vec::new();
    for direction in &directions {
        let new_point = point.as_ivec2() + direction.offset();
        if !map.contains_key(&new_point.as_uvec2()) {
            continue;
        }
        let new_height = *map.get(&new_point.as_uvec2()).unwrap();
        let old_height = *map.get(&point).unwrap();
        if old_height == 9 {
            continue;
        }
        if new_height == old_height + 1 {
            ret.push(new_point.as_uvec2());
        }
    }

    ret
}
fn main() {
    let inputs = include_str!("input.txt");
    let total = count_trailheads(&parse(inputs));
    println!("{:?}", total);
}
mod tests {
    use super::*;
    const EXAMPLE_ONE: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    const EXAMPLE_TWO: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const EXAMPLE_THREE: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    const EXAMPLE_FOUR: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn test_trailhead_count() {
        //assert_eq!(count_trailheads(&parse(&EXAMPLE_ONE)), 2);
        //assert_eq!(count_trailheads(&parse(&EXAMPLE_TWO)), 4);
        //assert_eq!(count_trailheads(&parse(&EXAMPLE_THREE)), 3);
        assert_eq!(count_trailheads(&parse(EXAMPLE_FOUR)), 81);
    }
}
