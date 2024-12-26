use day_01::parse_input;
fn main() {
    let inputs = include_str!("input.txt");
    let total = find_total_distance(inputs);
    println!("{:?}", total);
}

fn find_total_distance(input: &str) -> u32 {
    let mut total: u32 = 0;

    let (mut lhs, mut rhs) = parse_input(input);

    lhs.sort_unstable();
    rhs.sort_unstable();

    for (counter, lhs_number) in lhs.iter().enumerate() {
        let rhs_number = rhs[counter];
        let difference: i32 = *lhs_number as i32 - rhs_number as i32;
        if difference < 0 {
            total += -difference as u32;
        } else {
            total += difference as u32;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn find_correct_answer() {
        assert_eq!(11, find_total_distance(TEST_INPUT));
    }
    #[test]
    fn test_parsing_line() {
        let test_lhs: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let (lhs, _rhs) = parse_input(TEST_INPUT);
        assert_eq!(test_lhs, lhs);

        let test_rhs: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        let (_lhs, rhs) = parse_input(TEST_INPUT);
        assert_eq!(test_rhs, rhs);
    }
}
