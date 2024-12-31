use day_09::{calc_checksum, parse_input};
fn main() {
    let inputs = include_str!("input.txt");
    let decompressed_blocks = parse_input(inputs);
    let moved_blocks = move_blocks(&decompressed_blocks);
    let answer = calc_checksum(&moved_blocks);
    println!("{:?}", answer);
}

fn move_blocks(blocks: &Vec<Option<u32>>) -> Vec<Option<u32>> {
    let rev_blocks: Vec<u32> = blocks.iter().flatten().copied().collect();

    let mut file_size: Vec<(u32, u32, u32)> = Vec::new();
    let mut file_name = rev_blocks[0];
    let mut count: u32 = 0;
    let mut pos = 0;
    for file in &rev_blocks {
        if file_name == *file {
            count += 1;
        } else {
            file_size.push((file_name, count, pos as u32));
            count = 1;
            file_name = *file;
            pos = blocks.iter().position(|item| *item == Some(*file)).unwrap();
        }
    }
    file_size.push((file_name, count, pos as u32));
    let file_size: Vec<(u32, u32, u32)> = file_size.iter().rev().copied().collect();

    let mut compacted_blocks = blocks.clone();
    for file in file_size {
        if let Some(start_index) = compacted_blocks
            .windows(file.1 as usize)
            .position(|item| item == vec![None; file.1 as usize])
        {
            let orig_file_start = file.2 as usize;
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
