pub fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: Vec<u32> = Vec::new();

    for line in input.lines() {
        for (count, number) in line.split_whitespace().enumerate() {
            if count == 0 {
                lhs.push(number.parse::<u32>().unwrap());
            } else {
                rhs.push(number.parse::<u32>().unwrap());
            }
        }
    }

    (lhs, rhs)
}
