use itertools::Itertools;
const OPERATORS: [char; 3] = ['*', '+', '|'];
struct CalibrationEquation {
    answer: u64,
    numbers: Vec<u32>,
}

fn main() {
    let inputs = include_str!("input.txt");
    let total = count_total_calibration_result(inputs);
    println!("{:?}", total);
}
fn count_total_calibration_result(input: &str) -> u64 {
    let mut total = 0;
    let calibration_equations = parse_input(input);

    for equation in &calibration_equations {
        let operators = get_all_possible_operators_for_equation(equation.numbers.len() as u32 - 1);
        for operator in &operators {
            if can_answer_be_found(equation, operator) {
                total += equation.answer;
                break;
            }
        }
    }

    total
}
fn get_all_possible_operators_for_equation(no_of_numbers: u32) -> Vec<Vec<char>> {
    (0..no_of_numbers)
        .map(|_| OPERATORS)
        .multi_cartesian_product()
        .collect()
}

fn can_answer_be_found(equation: &CalibrationEquation, list_of_operator: &[char]) -> bool {
    let mut answer: u64;
    answer = u64::from(equation.numbers[0]);
    for number in equation.numbers.iter().skip(1).enumerate() {
        match list_of_operator[number.0] {
            '+' => answer += u64::from(*number.1),
            '*' => answer *= u64::from(*number.1),
            '|' => answer = format!("{}{}", answer, number.1).parse::<u64>().unwrap(),
            _ => {}
        }
    }
    answer == equation.answer
}
fn parse_input(input: &str) -> Vec<CalibrationEquation> {
    let mut return_equations: Vec<CalibrationEquation> = Vec::new();
    for line in input.lines() {
        let parse_out_answer = line.split(':').collect::<Vec<&str>>();
        let numbers = parse_out_answer[1]
            .trim()
            .split(' ')
            .map(|number| number.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        return_equations.push(CalibrationEquation {
            answer: parse_out_answer[0].parse::<u64>().unwrap(),
            numbers,
        });
    }

    return_equations
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn test_count_total_calibration_result() {
        assert_eq!(count_total_calibration_result(TEST_INPUT), 11387);
    }
    #[test]
    fn test_parse_input() {
        let parsed_input = parse_input(TEST_INPUT);
        assert_eq!(parsed_input.len(), 9);
        assert_eq!(parsed_input[3].answer, 156);
        assert_eq!(parsed_input[2].numbers[1], 5);
    }
}
