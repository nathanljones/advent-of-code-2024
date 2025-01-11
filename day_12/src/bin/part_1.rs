use glam::{IVec2, UVec2};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let inputs = include_str!("input.txt");
    let total = calc_price(inputs);
    println!("{total}");
}

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
}

fn calc_price(input: &str) -> u32 {
    let grid = parse_input(input);
    let grid_size = get_grid_size(input);
    let regions = get_regions(&grid, grid_size);

    regions.iter().fold(0, |acc: u32, region| {
        acc + (get_region_area(region) * get_region_perimeter(region))
    })
}
fn get_regions(grid: &HashMap<UVec2, char>, grid_size: (u32, u32)) -> Vec<HashMap<UVec2, char>> {
    let mut places_visited: HashSet<UVec2> = HashSet::new();
    let mut stack: VecDeque<UVec2> = vec![].into();
    let directions = build_directions();
    let mut current_region: HashMap<UVec2, char> = HashMap::new();
    let mut return_val: Vec<HashMap<UVec2, char>> = Vec::new();

    for row in 0..grid_size.1 {
        for col in 0..grid_size.0 {
            if places_visited.contains(&(UVec2::new(row, col))) {
                continue;
            }
            current_region.clear();
            let current_char = grid.get(&UVec2::new(row, col)).unwrap();
            current_region.insert(UVec2::new(row, col), *current_char);
            stack.push_back(UVec2::new(row, col));
            places_visited.insert(UVec2::new(row, col));

            while !stack.is_empty() {
                let current_plot = stack.pop_front().unwrap();
                for direction in &directions {
                    let new_grid_pos = current_plot.as_ivec2() + direction.offset();
                    // check we haven't gone negative
                    if new_grid_pos.abs() != new_grid_pos {
                        continue;
                    }
                    let new_grid_pos = new_grid_pos.as_uvec2();
                    if places_visited.contains(&(new_grid_pos)) {
                        continue;
                    }
                    if grid.contains_key(&new_grid_pos)
                        && grid.get(&new_grid_pos).unwrap() == current_char
                    {
                        current_region.insert(new_grid_pos, *current_char);
                        stack.push_back(new_grid_pos);
                        places_visited.insert(new_grid_pos);
                    }
                }
            }
            return_val.push(current_region.clone());
        }
    }

    return_val
}

fn get_region_area(region: &HashMap<UVec2, char>) -> u32 {
    u32::try_from(region.len()).unwrap()
}
fn get_region_perimeter(region: &HashMap<UVec2, char>) -> u32 {
    let directions = build_directions();
    region.iter().fold(0, |acc: u32, (pos, _char)| {
        acc + directions.iter().fold(0, |acc, direction| {
            let new_grid_pos = pos.as_ivec2() + direction.offset();
            // check we haven't gone negative
            if new_grid_pos.abs() != new_grid_pos {
                acc + 1
            } else {
                let new_grid_pos = new_grid_pos.as_uvec2();
                if region.contains_key(&new_grid_pos) {
                    acc
                } else {
                    acc + 1
                }
            }
        })
    } /* f */)
}

fn build_directions() -> Vec<Direction> {
    vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
}
fn parse_input(input: &str) -> HashMap<UVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    UVec2::new(u32::try_from(x).unwrap(), u32::try_from(y).unwrap()),
                    c,
                )
            })
        })
        .collect::<HashMap<UVec2, char>>()
}

fn get_grid_size(input: &str) -> (u32, u32) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    (u32::try_from(cols).unwrap(), u32::try_from(rows).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GARDEN1: &str = "AAAA
BBCD
BBCC
EEEC";
    const GARDEN2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const GARDEN3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_parse_input() {
        let result = parse_input(GARDEN1);
        dbg!(&result);
    }

    #[test]
    fn test_grid_size() {
        assert_eq!(get_grid_size(GARDEN1), (4, 4));
        assert_eq!(get_grid_size(GARDEN2), (5, 5));
        assert_eq!(get_grid_size(GARDEN3), (10, 10));
    }
    #[test]
    fn test_get_regions_1() {
        let grid = parse_input(GARDEN1);
        let grid_size = get_grid_size(GARDEN1);
        let regions = get_regions(&grid, grid_size);
        dbg!(&regions);
    }
    #[test]
    fn test_get_regions_2() {
        let grid = parse_input(GARDEN2);
        let grid_size = get_grid_size(GARDEN2);
        let regions = get_regions(&grid, grid_size);
        dbg!(&regions);
    }
    #[test]
    fn test_get_regions_3() {
        let grid = parse_input(GARDEN3);
        let grid_size = get_grid_size(GARDEN3);
        let regions = get_regions(&grid, grid_size);
        dbg!(&regions);
    }
    #[test]
    fn test_get_areas_1() {
        let grid = parse_input(GARDEN1);
        let grid_size = get_grid_size(GARDEN1);
        let regions = get_regions(&grid, grid_size);
        for region in regions {
            if region.values().all(|c| *c == 'A') {
                assert_eq!(get_region_area(&region), 4);
            }
            if region.values().all(|c| *c == 'B') {
                assert_eq!(get_region_area(&region), 4);
            }
            if region.values().all(|c| *c == 'C') {
                assert_eq!(get_region_area(&region), 4);
            }
            if region.values().all(|c| *c == 'D') {
                assert_eq!(get_region_area(&region), 1);
            }
            if region.values().all(|c| *c == 'E') {
                assert_eq!(get_region_area(&region), 3);
            }
        }
    }
    #[test]
    fn test_get_areas_3() {
        let grid = parse_input(GARDEN3);
        let grid_size = get_grid_size(GARDEN3);
        let regions = get_regions(&grid, grid_size);
        for region in regions {
            if region.values().all(|c| *c == 'R') {
                assert_eq!(get_region_area(&region), 12);
            }
            if region.values().all(|c| *c == 'F') {
                assert_eq!(get_region_area(&region), 10);
            }
            if region.values().all(|c| *c == 'V') {
                assert_eq!(get_region_area(&region), 13);
            }
            if region.values().all(|c| *c == 'J') {
                assert_eq!(get_region_area(&region), 11);
            }
            if region.values().all(|c| *c == 'E') {
                assert_eq!(get_region_area(&region), 13);
            }
            if region.values().all(|c| *c == 'M') {
                assert_eq!(get_region_area(&region), 5);
            }
            if region.values().all(|c| *c == 'S') {
                assert_eq!(get_region_area(&region), 3);
            }
        }
    }

    #[test]
    fn test_get_perimeters_1() {
        let grid = parse_input(GARDEN1);
        let grid_size = get_grid_size(GARDEN1);
        let regions = get_regions(&grid, grid_size);
        for region in regions {
            if region.values().all(|c| *c == 'A') {
                assert_eq!(get_region_perimeter(&region), 10);
            }
            if region.values().all(|c| *c == 'B') {
                assert_eq!(get_region_perimeter(&region), 8);
            }
            if region.values().all(|c| *c == 'C') {
                assert_eq!(get_region_perimeter(&region), 10);
            }
            if region.values().all(|c| *c == 'D') {
                assert_eq!(get_region_perimeter(&region), 4);
            }
            if region.values().all(|c| *c == 'E') {
                assert_eq!(get_region_perimeter(&region), 8);
            }
        }
    }
    #[test]
    fn test_cal_price() {
        assert_eq!(calc_price(GARDEN1), 140);
        assert_eq!(calc_price(GARDEN2), 772);
        assert_eq!(calc_price(GARDEN3), 1930);
    }
}
