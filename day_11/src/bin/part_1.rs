use std::collections::HashMap;

fn main() {
    let inputs = include_str!("input.txt");
    let total = count_stones(inputs, 75);
    println!("{:?}", total);
}
fn count_stones(input: &str, no_times: u32) -> u64 {
    let mut list_of_stones = parse_input(input);
    let mut new_list_of_stones: HashMap<String, u64> = HashMap::default();
    for _iteration in 0..no_times {
        new_list_of_stones.clear();
        for (stone_no, stone_count) in &list_of_stones {
            let new_stone_combo = convert_stone(stone_no);
            for (stone_value, count) in new_stone_combo.iter() {
                if new_list_of_stones.contains_key(stone_value) {
                    let val = new_list_of_stones.get_mut(stone_value).unwrap();
                    *val += count * stone_count;
                } else {
                    new_list_of_stones.insert(stone_value.to_owned(), count * stone_count);
                }
            }
        }
        list_of_stones = new_list_of_stones.clone();
    }

    list_of_stones.into_values().sum()
}
fn convert_stone(stone: &str) -> HashMap<String, u64> {
    let mut result: HashMap<String, u64> = HashMap::new();
    if stone == "0" {
        result.insert(String::from("1"), 1);
    } else if stone.len() % 2 == 0 {
        let (first_stone, second_stone) = stone.split_at(stone.len() / 2);
        let first_stone_no = first_stone.parse::<u32>().unwrap();
        let second_stone_no = second_stone.parse::<u32>().unwrap();
        if first_stone_no != second_stone_no {
            result.insert(first_stone_no.to_string(), 1);
            result.insert(second_stone_no.to_string(), 1);
        } else {
            result.insert(first_stone_no.to_string(), 2);
        }
    } else {
        let mut stone_no: u64 = stone.parse::<u64>().unwrap();
        stone_no *= 2024;
        result.insert(stone_no.to_string(), 1);
    }
    result
}
fn parse_input(input: &str) -> HashMap<String, u64> {
    let mut stones: HashMap<String, u64> = HashMap::new();
    for digit in input.split_whitespace().map(std::borrow::ToOwned::to_owned) {
        stones.insert(digit, 1);
    }
    stones
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "125 17";
    const TEST_INPUT_2: &str = "0 1 10 99 999";

    #[test]
    fn test_count_stones() {
        assert_eq!(count_stones(TEST_INPUT, 1), 3);
        assert_eq!(count_stones(TEST_INPUT, 2), 4);
        assert_eq!(count_stones(TEST_INPUT, 3), 5);
        assert_eq!(count_stones(TEST_INPUT, 4), 9);
        assert_eq!(count_stones(TEST_INPUT, 5), 13);
        assert_eq!(count_stones(TEST_INPUT, 6), 22);
        assert_eq!(count_stones(TEST_INPUT, 25), 55312);
    }
    #[test]
    fn test_2() {
        assert_eq!(count_stones(TEST_INPUT_2, 1), 7);
    }
    #[test]
    fn test_convert_stones() {
        let vec: Vec<String> = convert_stone("0").into_keys().collect();
        assert_eq!(vec, vec!["1"]);
        assert_eq!(convert_stone("0").into_values().sum::<u64>(), 1);
        let vec: Vec<String> = convert_stone("1").into_keys().collect();
        assert_eq!(vec, vec!["2024"]);
        assert_eq!(convert_stone("0").into_values().sum::<u64>(), 1);
        let mut vec: Vec<String> = convert_stone("10").into_keys().collect();
        vec.sort_unstable();
        assert_eq!(vec, vec!["0", "1"]);
        assert_eq!(convert_stone("10").into_values().sum::<u64>(), 2);
        let vec: Vec<String> = convert_stone("99").into_keys().collect();
        assert_eq!(vec, vec!["9"]);
        assert_eq!(convert_stone("99").into_values().sum::<u64>(), 2);
        let vec: Vec<String> = convert_stone("999").into_keys().collect();
        assert_eq!(vec, vec!["2021976"]);
        assert_eq!(convert_stone("0").into_values().sum::<u64>(), 1);
    }
}
