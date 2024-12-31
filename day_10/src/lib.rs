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

pub fn parse(input: &str) -> HashMap<UVec2, u32> {
    let mut ret: HashMap<UVec2, u32> = HashMap::new();
    for line in input.lines().enumerate() {
        for char in line.1.chars().enumerate() {
            if char.1.is_ascii_digit() {
                ret.insert(
                    UVec2::new(char.0.try_into().unwrap(), line.0.try_into().unwrap()),
                    char.1.to_digit(10).unwrap(),
                );
            }
        }
    }

    ret
}
