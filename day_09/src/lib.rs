pub fn calc_checksum(blocks: &[Option<u32>]) -> u64 {
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
pub fn parse_input(input: &str) -> Vec<Option<u32>> {
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
