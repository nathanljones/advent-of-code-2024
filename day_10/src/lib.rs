use glam::{IVec2, UVec2};
use std::collections::HashMap;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn offset(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }
}

#[must_use]
pub fn parse(input: &str) -> HashMap<UVec2, u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, character)| character.is_ascii_digit())
                .map(move |(x, char)| {
                    (
                        UVec2::new(x.try_into().unwrap(), y.try_into().unwrap()),
                        char.to_digit(10).unwrap(),
                    )
                })
        })
        .collect::<HashMap<UVec2, u32>>()
}
