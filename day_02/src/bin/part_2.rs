fn main() {
    let inputs = include_str!("input.txt");
    let total = find_no_safe_reports(inputs);
    println!("{:?}", total);
}
fn find_no_safe_reports(inputs: &str) -> u32 {
    let mut total: u32 = 0;
    for line in inputs.lines() {
        if is_safe(line) {
            total += 1;
            continue;
        }
        let numbers_count = line.split_whitespace().count();
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        let mut test_string: String = String::new();
        for loop_count in 0..numbers_count {
            for nos in 0..numbers_count {
                if loop_count != nos {
                    test_string = test_string + " " + numbers[nos];
                }
            }
            if is_safe(&test_string) {
                total += 1;
                continue;
            }
            test_string = String::new();
        }
    }
    total
}
fn is_safe(line: &str) -> bool {
    let mut is_ascending = false;
    let mut first_no: u32 = 0;
    let mut previous_no: u32 = 0;
    let parsed_data = line.split_whitespace().collect::<Vec<&str>>();
    // first pass check if ascending or descending
    for (count, number) in parsed_data.iter().enumerate() {
        if count == 0 {
            first_no = number.parse::<u32>().unwrap();
            previous_no = number.parse::<u32>().unwrap();
            continue;
        }
        if count == 1 {
            is_ascending = number.parse::<u32>().unwrap() > first_no;
            if previous_no.abs_diff(number.parse::<u32>().unwrap()) > 3 {
                return false;
            }
            if previous_no == number.parse::<u32>().unwrap() {
                return false;
            }

            previous_no = number.parse::<u32>().unwrap();
            continue;
        }
        if previous_no == number.parse::<u32>().unwrap() {
            return false;
        }
        if previous_no >= number.parse::<u32>().unwrap() && is_ascending {
            return false;
        }
        if previous_no <= number.parse::<u32>().unwrap() && !is_ascending {
            return false;
        }

        if previous_no.abs_diff(number.parse::<u32>().unwrap()) > 3 {
            return false;
        }
        previous_no = number.parse::<u32>().unwrap();
    }

    true
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn test_no_safe_reports() {
        assert_eq!(4, find_no_safe_reports(TEST_INPUT));
    }
    #[test]
    fn test_is_safe() {
        let input: &str = "7 6 4 2 1";
        assert!(is_safe(input));
        let input: &str = "1 2 7 8 9";
        assert!(!is_safe(input));
        let input: &str = "9 7 6 2 1";
        assert!(!is_safe(input));
        let input: &str = "1 3 2 4 5";
        assert!(!is_safe(input));
        let input: &str = "8 6 4 4 1";
        assert!(!is_safe(input));
        let input: &str = "1 3 6 7 9";
        assert!(is_safe(input));
        let input: &str = "1 5 6 7 9";
        assert!(!is_safe(input));
        let input: &str = "1 1 3 6 7 9";
        assert!(!is_safe(input));
    }
}
