fn main() {
    let inputs = include_str!("input.txt");
    let total = find_similarity_score(inputs);
    println!("{:?}", total);
}

fn find_similarity_score(input: &str) -> u32 {
    let mut total: u32 = 0;

    let (mut lhs, mut rhs) = parse_input(input);

    for number in lhs {
        let how_many_times_do_numbers_appear = rhs.iter().filter(|mut x| **x == number).count();
        total = total + (number * how_many_times_do_numbers_appear as u32);
    }

    total
}

fn find_total_distance(input: &str) -> u32 {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: Vec<u32> = Vec::new();
    let mut total: u32 = 0;

    let (mut lhs, mut rhs) = parse_input(input);

    lhs.sort();
    rhs.sort();

    for (counter, lhs_number) in lhs.iter().enumerate() {
        let rhs_number = rhs[counter];
        let difference: i32 = *lhs_number as i32 - rhs_number as i32;
        if difference < 0 {
            total += (difference * -1) as u32;
        } else {
            total += difference as u32;
        }
    }
    total
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
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

    return (lhs, rhs);
}
mod tests {
    use super::*;
    const test_input: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn find_correct_answer() {
        assert_eq!(11, find_total_distance(&test_input));
    }
    #[test]
    fn test_parsing_line() {
        let test_lhs: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let (lhs, rhs) = parse_input(test_input);
        assert_eq!(test_lhs, lhs);

        let test_rhs: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        let (lhs, rhs) = parse_input(test_input);
        assert_eq!(test_rhs, rhs);
    }

    #[test]
    fn test_find_similarity_score() {
        assert_eq!(31, find_similarity_score(&test_input));
    }
}
