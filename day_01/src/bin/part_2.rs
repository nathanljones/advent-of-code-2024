use day_01::parse_input;
fn main() {
    let inputs = include_str!("input.txt");
    let total = find_similarity_score(inputs);
    println!("{:?}", total);
}

fn find_similarity_score(input: &str) -> u32 {
    let mut total: u32 = 0;

    let (lhs, rhs) = parse_input(input);

    for number in lhs {
        let how_many_times_do_numbers_appear = rhs.iter().filter(|x| **x == number).count();
        total += number * how_many_times_do_numbers_appear as u32;
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
    fn test_parsing_line() {
        let test_lhs: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let (lhs, _rhs) = parse_input(TEST_INPUT);
        assert_eq!(test_lhs, lhs);

        let test_rhs: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        let (_lhs, rhs) = parse_input(TEST_INPUT);
        assert_eq!(test_rhs, rhs);
    }

    #[test]
    fn test_find_similarity_score() {
        assert_eq!(31, find_similarity_score(TEST_INPUT));
    }
}
