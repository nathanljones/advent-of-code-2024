use glam::{IVec2, UVec2};

fn main() {
    let inputs = include_str!("input.txt");
    let total = how_many_times_to_appear(inputs);
    println!("{:?}", total);
}
enum Direction {
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}
impl Direction {
    pub fn offset(&self) -> IVec2 {
        match self {
            Direction::LeftUp => IVec2::new(-1, -1),
            Direction::RightUp => IVec2::new(1, -1),
            Direction::LeftDown => IVec2::new(-1, 1),
            Direction::RightDown => IVec2::new(1, 1),
        }
    }
}

fn how_many_times_to_appear(input: &str) -> u32 {
    let search_grid = convert_to_grid(input);
    let mut first_pos: UVec2 = UVec2::new(0, 0);
    let mut new_pos: IVec2 = IVec2::new(0, 0);
    let mut total: u32 = 0;
    let mut char_to_search_for: char;

    for line in &search_grid {
        for char in line {
            if *char != 'A' {
                first_pos.x += 1;
                continue;
            }
            let direction = Direction::LeftUp;
            new_pos.x = first_pos.x as i32;
            new_pos.y = first_pos.y as i32;
            new_pos.x += direction.offset().x;
            new_pos.y += direction.offset().y;

            if !is_new_pos_out_of_bounds(
                new_pos,
                (line.len() - 1) as i32,
                (search_grid.len() - 1) as i32,
            ) {
                first_pos.x += 1;
                continue;
            }

            if search_grid[new_pos.y as usize][new_pos.x as usize] == 'M' {
                char_to_search_for = 'S';
            } else if search_grid[new_pos.y as usize][new_pos.x as usize] == 'S' {
                char_to_search_for = 'M';
            } else {
                first_pos.x += 1;
                continue;
            }

            let direction = Direction::RightDown;
            new_pos.x = first_pos.x as i32;
            new_pos.y = first_pos.y as i32;
            new_pos.x += direction.offset().x;
            new_pos.y += direction.offset().y;

            if !is_new_pos_out_of_bounds(
                new_pos,
                (line.len() - 1) as i32,
                (search_grid.len() - 1) as i32,
            ) {
                first_pos.x += 1;
                continue;
            }
            if search_grid[new_pos.y as usize][new_pos.x as usize] != char_to_search_for {
                first_pos.x += 1;
                continue;
            }

            let direction = Direction::RightUp;
            new_pos.x = first_pos.x as i32;
            new_pos.y = first_pos.y as i32;
            new_pos.x += direction.offset().x;
            new_pos.y += direction.offset().y;

            if !is_new_pos_out_of_bounds(
                new_pos,
                (line.len() - 1) as i32,
                (search_grid.len() - 1) as i32,
            ) {
                first_pos.x += 1;
                continue;
            }

            if search_grid[new_pos.y as usize][new_pos.x as usize] == 'M' {
                char_to_search_for = 'S';
            } else if search_grid[new_pos.y as usize][new_pos.x as usize] == 'S' {
                char_to_search_for = 'M';
            } else {
                first_pos.x += 1;
                continue;
            }

            let direction = Direction::LeftDown;
            new_pos.x = first_pos.x as i32;
            new_pos.y = first_pos.y as i32;
            new_pos.x += direction.offset().x;
            new_pos.y += direction.offset().y;

            if !is_new_pos_out_of_bounds(
                new_pos,
                (line.len() - 1) as i32,
                (search_grid.len() - 1) as i32,
            ) {
                first_pos.x += 1;
                continue;
            }
            if search_grid[new_pos.y as usize][new_pos.x as usize] != char_to_search_for {
                first_pos.x += 1;
                continue;
            }
            total += 1;
            first_pos.x += 1;
        }
        first_pos.x = 0;
        first_pos.y += 1;
    }
    total
}
fn is_new_pos_out_of_bounds(position: IVec2, upper_bound_x: i32, upper_bound_y: i32) -> bool {
    if position.x < 0 {
        return false;
    }
    if position.y < 0 {
        return false;
    }
    if position.x > upper_bound_x {
        return false;
    }
    if position.y > upper_bound_y {
        return false;
    }

    true
}
fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_how_many_times() {
        assert_eq!(9, how_many_times_to_appear(TEST_INPUT));
    }
}
