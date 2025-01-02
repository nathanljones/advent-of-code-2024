use regex::Regex;

fn main() {
    let inputs = include_str!("input.txt");
    let total = add_result_of_instruction_process(inputs);
    println!("{total}");
}

fn add_result_of_instruction_process(input: &str) -> u32 {
    let re_instruction = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let instructions: Vec<&str> = re_instruction
        .find_iter(input)
        .map(|m| m.as_str())
        .collect();
    instructions.iter().fold(0, |acc, instruction| {
        let parsed_instruction = instruction.trim_start_matches("mul(");
        let parsed_instruction = parsed_instruction.trim_end_matches(')');
        let numbers = parsed_instruction
            .split(',')
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        acc + numbers[0] * numbers[1]
    })
}
#[cfg(test)]
mod tests {
    use crate::add_result_of_instruction_process;

    const TEST_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    #[test]
    fn test_add_instruction_process() {
        assert_eq!(161, add_result_of_instruction_process(TEST_STR));
    }
}
