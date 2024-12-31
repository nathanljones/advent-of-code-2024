fn main() {
    let inputs = include_str!("input.txt");
    let decompressed_blocks = parse_input(inputs);
    let moved_blocks = move_blocks(&decompressed_blocks);
    let answer = calc_checksum(&moved_blocks);
    println!("{:?}", answer);
}

fn move_blocks(blocks: &Vec<Option<u32>>) -> Vec<Option<u32>> {
    let rev_blocks: Vec<u32> = blocks
        .iter()
        .rev()
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect();

    let mut file_size: Vec<(u32, u32)> = Vec::new();
    let mut file_name = rev_blocks[0];
    let mut count: u32 = 0;
    for file in rev_blocks {
        if file_name == file {
            count += 1;
        } else {
            file_size.push((file_name, count));
            count = 1;
            file_name = file;
        }
    }
    file_size.push((file_name, count));

    let mut compacted_blocks = blocks.clone();
    for file in file_size {
        if let Some(start_index) = compacted_blocks
            .windows(file.1 as usize)
            .position(|item| item == vec![None; file.1 as usize])
        {
            let orig_file_start = blocks
                .iter()
                .position(|item| *item == Some(file.0))
                .unwrap();
            if start_index < orig_file_start {
                for pos in 0..file.1 {
                    compacted_blocks[start_index + pos as usize] = Some(file.0);
                }

                let clear_end = orig_file_start + file.1 as usize;
                for pos in orig_file_start..clear_end {
                    compacted_blocks[pos] = None;
                }
            }
        }
    }
    compacted_blocks
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
    fn move_blocks_complex() {
        let test_output: Vec<Option<u32>> = vec![
            Some(0),
            Some(0),
            Some(9),
            Some(9),
            Some(2),
            Some(1),
            Some(1),
            Some(1),
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(4),
            Some(4),
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            None,
            None,
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            None,
            None,
            None,
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            None,
            None,
        ];
        let decompressed_blocks = parse_input(COMPLEX_INPUT);
        assert_eq!(test_output, move_blocks(&decompressed_blocks));
    }
    #[test]
    fn calc_checksum_complex() {
        let decompressed_blocks = parse_input(COMPLEX_INPUT);
        assert_eq!(calc_checksum(&move_blocks(&decompressed_blocks)), 2858);
    }
}
