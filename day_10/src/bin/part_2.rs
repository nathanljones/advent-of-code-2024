use day_10::{parse, Direction};
use glam::UVec2;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

fn count_trailheads(map: &HashMap<UVec2, u32>) -> u32 {
    let start_positions: Vec<(UVec2, u32)> =
        map.clone().into_iter().filter(|pos| pos.1 == 0).collect();
    let end_positions: Vec<(UVec2, u32)> =
        map.clone().into_iter().filter(|pos| pos.1 == 9).collect();
    start_positions.iter().fold(0, |acc, start_pos| {
        acc + end_positions.iter().fold(0, |acc, pos| {
            acc + u32::try_from(count_paths(
                start_pos.0,
                |&p| get_next_nodes(p, map).into_iter(),
                |&p| p == pos.0,
            ))
            .unwrap()
        })
    })
}

fn get_next_nodes(point: UVec2, map: &HashMap<UVec2, u32>) -> Vec<UVec2> {
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    directions
        .iter()
        .flat_map(|direction| {
            let mut ret: Vec<UVec2> = Vec::new();
            let new_point = point.as_ivec2() + direction.offset();
            if map.contains_key(&new_point.as_uvec2()) {
                let new_height = *map.get(&new_point.as_uvec2()).unwrap();
                let old_height = *map.get(&point).unwrap();
                if old_height < 9 && new_height == old_height + 1 {
                    ret.push(new_point.as_uvec2());
                }
            }
            ret.into_iter()
        })
        .collect()
}
fn main() {
    let inputs = include_str!("input.txt");
    let total = count_trailheads(&parse(inputs));
    println!("{total}");
}
#[cfg(test)]
mod tests {
    use super::*;
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
