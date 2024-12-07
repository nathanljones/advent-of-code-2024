use glam::{IVec2, UVec2};

fn main() {
    let inputs = include_str!("input.txt");
    let total = how_many_times_to_appear(inputs);
    println!("{:?}", total);
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}
impl Direction {
    pub fn offset(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, -1),
            Direction::Down => IVec2::new(0, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
            Direction::LeftUp => IVec2::new(-1, -1),
            Direction::RightUp => IVec2::new(1, -1),
            Direction::LeftDown => IVec2::new(-1, 1),
            Direction::RightDown => IVec2::new(1, 1),
        }
    }
}

fn how_many_times_to_appear(input: &str) -> u32 {
    let search_grid = convert_to_grid(input);
    let chars_to_find: Vec<char> = vec!['M', 'A', 'S'];
    let directions = build_directions();
    let mut search_chars_count: u32 = 0;
    let mut first_pos: UVec2 = UVec2::new(0, 0);
    let mut new_pos: IVec2 = IVec2::new(0, 0);
    let mut total: u32 = 0;

    for line in search_grid.iter() {
        for char in line {
            if *char != 'X' {
                first_pos.x += 1;
                continue;
            }
            for direction in directions.iter() {
                new_pos.x = first_pos.x as i32;
                new_pos.y = first_pos.y as i32;
                search_chars_count = 0;
                for search_char in chars_to_find.iter() {
                    new_pos.x += direction.offset().x;
                    new_pos.y += direction.offset().y;

                    if new_pos.x < 0 {
                        continue;
                    }
                    if new_pos.y < 0 {
                        continue;
                    }
                    if new_pos.x > (line.len() - 1) as i32 {
                        continue;
                    }
                    if new_pos.y > (search_grid.len() - 1) as i32 {
                        continue;
                    }
                    if search_grid[new_pos.y as usize][new_pos.x as usize] != *search_char {
                        search_chars_count = 0;
                        break;
                    }
                    search_chars_count += 1;
                }
                if search_chars_count == 3 {
                    total += 1;
                }
            }
            first_pos.x += 1;
        }
        first_pos.y += 1;
        first_pos.x = 0;
    }
    total
}
fn convert_to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn build_directions() -> Vec<Direction> {
    vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::LeftUp,
        Direction::RightUp,
        Direction::RightDown,
        Direction::LeftDown,
    ]
}
mod tests {
    use crate::how_many_times_to_appear;

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
        assert_eq!(18, how_many_times_to_appear(TEST_INPUT));
    }
}
