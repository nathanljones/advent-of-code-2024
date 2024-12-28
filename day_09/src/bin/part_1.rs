use std::collections::VecDeque;
fn main() {
    let inputs = include_str!("input.txt");
    let decompressed_blocks = parse_input(inputs);
    let moved_blocks = move_blocks(&decompressed_blocks);
    let answer = calc_checksum(&moved_blocks);
    println!("{:?}", answer);
}

fn move_blocks(blocks: &[Option<u32>]) -> Vec<Option<u32>> {
    let mut rev_blocks: VecDeque<Option<u32>> = blocks
        .iter()
        .rev()
        .filter(|item| item.is_some())
        .copied()
        .collect();
    let no_free_spaces = blocks.iter().filter(|item| item.is_none()).count();
    let free_space_index = blocks.len() - no_free_spaces;
    blocks
        .iter()
        .enumerate()
        .map(|item| {
            let ret_value: Option<u32>;
            if item.0 >= free_space_index {
                ret_value = None;
            } else if item.1.is_some() || rev_blocks.is_empty() {
                ret_value = *item.1;
            } else {
                ret_value = rev_blocks[0];
                rev_blocks.pop_front();
            }
            ret_value
        })
        .collect()
}

fn calc_checksum(blocks: &[Option<u32>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|value| {
            if value.1.is_some() {
                value.0 as u64 * u64::from(value.1.unwrap())
            } else {
                0
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut file_id: u32 = 0;
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let count: u32 = c.to_digit(10).unwrap();
            let mut test_output: Vec<Option<u32>> = vec![];
            for _x in 0..count {
                if i % 2 != 0 && i > 0 {
                    test_output.push(None);
                } else {
                    test_output.push(Some(file_id));
                }
            }
            if i % 2 != 0 && i > 0 {
                file_id += 1;
            }
            test_output.into_iter()
        })
        .collect::<Vec<Option<u32>>>()
}

mod tests {
    use super::*;
    const SIMPLE_TEST_INPUT: &str = "12345";
    const COMPLEX_INPUT: &str = "2333133121414131402";
    #[test]
    fn test_parse_input_simple() {
        let test_output: Vec<Option<u32>> = vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ];
        assert_eq!(parse_input(SIMPLE_TEST_INPUT), test_output);
    }
    #[test]
    fn move_blocks_simple() {
        let test_output: Vec<Option<u32>> = vec![
            Some(0),
            Some(2),
            Some(2),
            Some(1),
            Some(1),
            Some(1),
            Some(2),
            Some(2),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let decompressed_blocks = parse_input(SIMPLE_TEST_INPUT);
        assert_eq!(test_output, move_blocks(&decompressed_blocks));
    }
    #[test]
    fn calc_checksum_simple() {
        let decompressed_blocks = parse_input(COMPLEX_INPUT);
        assert_eq!(calc_checksum(&move_blocks(&decompressed_blocks)), 1928);
    }
}
