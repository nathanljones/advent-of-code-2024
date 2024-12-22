use glam::{IVec2, UVec2};
use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
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
    pub fn new_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn main() {
    let inputs = include_str!("input.txt");
    let total = count_no_obstacles(inputs);
    println!("{:?}", total);
}
fn count_distinct_positions(input: &str) -> u32 {
    let mut map = parse(input);
    let grid_size = get_grid_size(input);
    let start_position = get_current_position(&map);
    let mut current_position = start_position;
    let mut current_direction = Direction::Up;

    while !will_leave_grid(current_position, &current_direction, grid_size) {
        map.insert(current_position, 'X');
        if will_hit_object(current_position, &current_direction, &map) {
            current_direction = current_direction.new_direction();
        }
        current_position = move_guard(current_position, &current_direction);
    }

    map.iter().filter(|&(_, v)| *v == 'X').count() as u32 + 1
}
fn count_no_obstacles(input: &str) -> u64 {
    let map = parse(input);
    let grid_size = get_grid_size(input);
    let start_position = get_current_position(&map);
    let mut current_position = start_position;
    let mut current_direction = Direction::Up;
    let mut guard_route: HashMap<UVec2, char> = HashMap::new();
    let mut total_loops: u64 = 0;

    for y in (0..grid_size.y) {
        for x in (0..grid_size.x) {
            if start_position.x == x && start_position.y == y {
                continue;
            }
            let mut new_map = map.clone();
            new_map.insert(UVec2::new(x, y), '#');
            if will_form_a_loop(&new_map, grid_size) {
                //println!("X = {}, Y = {}", cell.0.x, cell.0.y);
                total_loops += 1;
                continue;
            }
        }
    }

    /*// first find the route guard will go on
    while !will_leave_grid(current_position, &current_direction, grid_size) {
        guard_route.insert(current_position, 'X');
        if will_hit_object(current_position, &current_direction, &map) {
            current_direction = current_direction.new_direction();
        }
        current_position = move_guard(current_position, &current_direction);
    }
    guard_route.insert(current_position, 'X');
    for cell in guard_route.iter() {
        if *cell.0 == start_position {
            continue;
        }

        let mut new_map = map.clone();
        new_map.insert(*cell.0, '#');
        if will_form_a_loop(&new_map, grid_size) {
            //println!("X = {}, Y = {}", cell.0.x, cell.0.y);
            total_loops += 1;
        }
    }*/
    total_loops
}

fn will_form_a_loop(map: &HashMap<UVec2, char>, grid_size: UVec2) -> bool {
    let start_position = get_current_position(&map);
    let mut current_position = start_position;
    let mut current_direction = Direction::Up;
    let mut guard_route: HashMap<UVec2, (char, Vec<Direction>)> = HashMap::new();
    let mut count: u32 = 0;

    while !will_leave_grid(current_position, &current_direction, grid_size) {
        if count > 25000 {
            return true;
        }
        count += 1;
        /*if let Some(current_cell) = guard_route.get(&current_position) {
            if (current_cell.0 == 'X') && current_cell.1.contains(&current_direction) {
                return true;
            } else if current_cell.0 == 'X' {
                let mut new_directions = current_cell.1.clone();
                new_directions.push(current_direction);
                guard_route.insert(current_position, ('X', new_directions));
            }
        } else {
            let mut new_directions = Vec::new();
            new_directions.push(current_direction);
            guard_route.insert(current_position, ('X', new_directions));
        } */
        if will_hit_object(current_position, &current_direction, &map) {
            current_direction = current_direction.new_direction();
        }
        current_position = move_guard(current_position, &current_direction);
    }

    false
}
fn will_leave_grid(current_pos: UVec2, direction: &Direction, grid_size: UVec2) -> bool {
    let mut result: bool = false;
    match direction {
        Direction::Up => {
            if current_pos.y == 0 {
                result = true;
            };
        }
        Direction::Down => {
            if current_pos.y == grid_size.y - 1 {
                result = true;
            };
        }
        Direction::Left => {
            if current_pos.x == 0 {
                result = true;
            };
        }
        Direction::Right => {
            if current_pos.x == grid_size.x - 1 {
                result = true;
            };
        }
    }
    result
}

fn move_guard(current_pos: UVec2, direction: &Direction) -> UVec2 {
    let new_pos = current_pos.as_ivec2() + direction.offset();
    new_pos.as_uvec2()
}
fn will_hit_object(current_pos: UVec2, direction: &Direction, map: &HashMap<UVec2, char>) -> bool {
    let new_pos = current_pos.as_ivec2() + direction.offset();
    let new_pos = new_pos.as_uvec2();
    if let Some(c) = map.get(&new_pos) {
        *c == '#'
    } else {
        false
    }
}

fn get_current_position(map: &HashMap<UVec2, char>) -> UVec2 {
    let guard_positions: Vec<_> = map.iter().filter(|&(_, v)| *v == '^').collect();
    *guard_positions[0].0
}
fn parse(input: &str) -> HashMap<UVec2, char> {
    let mut map: HashMap<UVec2, char> = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        for (pos, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            map.insert(
                UVec2::new(pos.try_into().unwrap(), row.try_into().unwrap()),
                char,
            );
        }
    }
    map
}

fn get_grid_size(input: &str) -> UVec2 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    UVec2::new(rows.try_into().unwrap(), cols.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn test_parse() {
        let grid = parse(TEST_INPUT);
        assert_eq!(grid.get(&UVec2 { x: 9, y: 1 }), Some(&'#'));
    }
    #[test]
    fn test_current_position() {
        let grid = parse(TEST_INPUT);
        assert_eq!(get_current_position(&grid), UVec2::new(4, 6));
    }
    #[test]
    fn test_get_grid_size() {
        assert_eq!(get_grid_size(TEST_INPUT), UVec2::new(10, 10));
    }

    #[test]
    fn test_will_leave_grid() {
        let grid_size = get_grid_size(TEST_INPUT);
        let position = UVec2::new(0, 0);
        assert!(will_leave_grid(position, &Direction::Up, grid_size));
        assert!(will_leave_grid(position, &Direction::Left, grid_size));
        assert!(!will_leave_grid(position, &Direction::Down, grid_size));
        assert!(!will_leave_grid(position, &Direction::Right, grid_size));
        let position = UVec2::new(9, 9);
        assert!(!will_leave_grid(position, &Direction::Up, grid_size));
        assert!(!will_leave_grid(position, &Direction::Left, grid_size));
        assert!(will_leave_grid(position, &Direction::Down, grid_size));
        assert!(will_leave_grid(position, &Direction::Right, grid_size));
    }
    #[test]
    fn test_will_hit_object() {
        let position = UVec2::new(9, 2);
        let grid = parse(TEST_INPUT);
        assert!(will_hit_object(position, &Direction::Up, &grid));
        assert!(!will_hit_object(position, &Direction::Down, &grid));
        let position = UVec2::new(3, 3);
        assert!(will_hit_object(position, &Direction::Left, &grid));
        assert!(!will_hit_object(position, &Direction::Up, &grid));
    }

    #[test]
    fn test_move_guard() {
        let position = UVec2::new(9, 2);
        assert_eq!(move_guard(position, &Direction::Up), UVec2::new(9, 1));
        assert_eq!(move_guard(position, &Direction::Down), UVec2::new(9, 3));
        assert_eq!(move_guard(position, &Direction::Left), UVec2::new(8, 2));
        assert_eq!(move_guard(position, &Direction::Right), UVec2::new(10, 2));
    }
    #[test]
    fn test_count_distinct_positions() {
        assert_eq!(count_distinct_positions(TEST_INPUT), 41);
    }

    #[test]
    fn test_count_no_obstacles() {
        assert_eq!(count_no_obstacles(TEST_INPUT), 6);
    }

    #[test]
    fn test_will_form_a_loop() {
        let grid = parse(TEST_INPUT);
        let grid_size = get_grid_size(TEST_INPUT);
        assert!(!will_form_a_loop(&grid, grid_size));
        let mut new_map: HashMap<UVec2, char> = grid.clone();
        new_map.insert((0, 0).into(), '#');
        assert!(!will_form_a_loop(&new_map, grid_size));
        let mut new_map = grid.clone();
        new_map.insert((3, 6).into(), '#');
        assert!(will_form_a_loop(&new_map, grid_size));
        let mut new_map = grid.clone();
        new_map.insert((5, 6).into(), '#');
        assert!(!will_form_a_loop(&new_map, grid_size));
    }
}
