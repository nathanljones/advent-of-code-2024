#[derive(Clone, Copy)]
struct PageOrderingRules {
    page_number_before: u32,
    page_number: u32,
}
fn main() {
    let inputs = include_str!("input.txt");
    let total = add_up_middle_pages(inputs);
    println!("{:?}", total);
}

fn add_up_middle_pages(input: &str) -> u32 {
    let (page_ordering_rules, pages_list) = parse_input(input);
    pages_list
        .iter()
        .filter(|pages| are_pages_in_correct_order(&pages, &page_ordering_rules) == false)
        .map(|pages| {
            let sorted_pages = sort_paages_into_correct_order(&pages, &page_ordering_rules);
            find_middle_page_no(&sorted_pages)
        })
        .sum()
}
fn parse_input(input: &str) -> (Vec<PageOrderingRules>, Vec<Vec<u32>>) {
    let mut page_ordering_rules: Vec<PageOrderingRules> = vec![];
    let mut pages: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        if line.contains("|") {
            let ordering_rule = line.split("|").collect::<Vec<&str>>();
            let mut page_order: PageOrderingRules = PageOrderingRules {
                page_number_before: 0,
                page_number: 0,
            };
            page_order.page_number_before = ordering_rule[0].parse::<u32>().unwrap();
            page_order.page_number = ordering_rule[1].parse::<u32>().unwrap();
            page_ordering_rules.push(page_order);
        } else if line.contains(",") {
            let page_nos: Vec<u32> = line
                .split(",")
                .map(|no| no.parse::<u32>().unwrap())
                .collect();
            pages.push(page_nos);
        } else {
            continue;
        }
    }

    (page_ordering_rules, pages)
}
fn find_middle_page_no(pages: &[u32]) -> u32 {
    let middle_page_no = (pages.len() / 2);
    pages[middle_page_no]
}

fn are_pages_in_correct_order(pages: &[u32], page_ordering_rules: &[PageOrderingRules]) -> bool {
    for (position, page) in pages.iter().enumerate() {
        let page_order = filter_page_rules(*page, &page_ordering_rules);
        for pos in 0..position {
            let page_to_check = pages[pos];
            if page_order.contains(&page_to_check) {
                return false;
            }
        }
    }

    true
}

fn filter_page_rules(page_no: u32, page_ordering_rules: &[PageOrderingRules]) -> Vec<u32> {
    page_ordering_rules
        .iter()
        .filter(|page_ordering_rule| page_ordering_rule.page_number_before == page_no)
        .map(|page_ordering_rule: &PageOrderingRules| page_ordering_rule.page_number)
        .collect()
}

fn sort_paages_into_correct_order(
    pages: &[u32],
    page_ordering_rules: &[PageOrderingRules],
) -> Vec<u32> {
    let mut sorted_pages: Vec<u32> = pages.into();

    for (page_no, page) in pages.iter().enumerate() {
        let filtered_pages = filter_page_rules(*page, page_ordering_rules);
        let (smallest_page_index, found) =
            find_smallest_index_for_page(&sorted_pages, &filtered_pages);
        if found == false {
            continue;
        };
        let page_to_move_index = sorted_pages
            .iter()
            .position(|page_to_find| page_to_find == page);
        match page_to_move_index {
            Some(page_to_move) => {
                if page_to_move as u32 > smallest_page_index {
                    sorted_pages.remove(page_to_move as usize);
                    sorted_pages.insert(smallest_page_index as usize, *page);
                };
            }
            None => continue,
        }
    }

    sorted_pages
}
fn find_smallest_index_for_page(pages: &[u32], filtered_pages: &[u32]) -> (u32, bool) {
    let mut index: u32 = pages.len().try_into().unwrap();
    let mut found: bool = false;

    for page_to_find in filtered_pages {
        let pos_found = pages.iter().position(|page| page == page_to_find);
        match pos_found {
            Some(page_to_find) => {
                if (page_to_find as u32) < index {
                    index = page_to_find as u32;
                };
                found = true;
            }
            None => continue,
        }
    }
    if found {
        (index, found)
    } else {
        (0, found)
    }
}
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    #[test]
    fn test_parse_input() {
        let (page_ordering_rules, pages) = parse_input(TEST_INPUT);
        assert_eq!(21, page_ordering_rules.len());
        assert_eq!(6, pages.len())
    }
    #[test]
    fn test_middle_page_no() {
        let (page_ordering_rules, pages) = parse_input(TEST_INPUT);
        assert_eq!(61, find_middle_page_no(&pages[0]));
        assert_eq!(53, find_middle_page_no(&pages[1]));
        assert_eq!(29, find_middle_page_no(&pages[2]));
    }
    #[test]
    fn test_ordering_of_pages() {
        let (page_ordering_rules, pages) = parse_input(TEST_INPUT);
        assert_eq!(
            true,
            are_pages_in_correct_order(&pages[0], &page_ordering_rules)
        );
        assert_eq!(
            true,
            are_pages_in_correct_order(&pages[1], &page_ordering_rules)
        );
        assert_eq!(
            true,
            are_pages_in_correct_order(&pages[2], &page_ordering_rules)
        );
        assert_eq!(
            false,
            are_pages_in_correct_order(&pages[3], &page_ordering_rules)
        );
        assert_eq!(
            false,
            are_pages_in_correct_order(&pages[4], &page_ordering_rules)
        );
        assert_eq!(
            are_pages_in_correct_order(&pages[5], &page_ordering_rules),
            false
        );
    }
    #[test]
    fn test_filter_page_rules() {
        let (page_ordering_rules, pages) = parse_input(TEST_INPUT);
        let filtered_pages = filter_page_rules(97, &page_ordering_rules);
        assert_eq!(filtered_pages.len(), 6);
        assert_eq!(filtered_pages.contains(&13), true);
        let filtered_pages = filter_page_rules(97, &page_ordering_rules);
        assert_eq!(filtered_pages.len(), 6);
        assert_eq!(filtered_pages.contains(&75), true);
    }
    #[test]
    fn test_sort_pages_into_order() {
        let (page_ordering_rules, pages) = parse_input(TEST_INPUT);
        let correct_pages: Vec<u32> = vec![97, 75, 47, 61, 53];
        assert_eq!(
            sort_paages_into_correct_order(&pages[3], &page_ordering_rules,),
            correct_pages
        );
        let correct_pages: Vec<u32> = vec![61, 29, 13];
        assert_eq!(
            sort_paages_into_correct_order(&pages[4], &page_ordering_rules,),
            correct_pages
        );

        let correct_pages: Vec<u32> = vec![97, 75, 47, 29, 13];
        assert_eq!(
            sort_paages_into_correct_order(&pages[5], &page_ordering_rules,),
            correct_pages
        );
    }
}
