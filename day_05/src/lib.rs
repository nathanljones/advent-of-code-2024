#[derive(Clone, Copy)]
pub struct PageOrderingRules {
    page_number_before: u32,
    page_number: u32,
}
pub fn parse_input(input: &str) -> (Vec<PageOrderingRules>, Vec<Vec<u32>>) {
    let mut page_ordering_rules: Vec<PageOrderingRules> = vec![];
    let mut pages: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        if line.contains('|') {
            let ordering_rule = line.split('|').collect::<Vec<&str>>();
            let mut page_order: PageOrderingRules = PageOrderingRules {
                page_number_before: 0,
                page_number: 0,
            };
            page_order.page_number_before = ordering_rule[0].parse::<u32>().unwrap();
            page_order.page_number = ordering_rule[1].parse::<u32>().unwrap();
            page_ordering_rules.push(page_order);
        } else if line.contains(',') {
            let page_nos: Vec<u32> = line
                .split(',')
                .map(|no| no.parse::<u32>().unwrap())
                .collect();
            pages.push(page_nos);
        } else {
            continue;
        }
    }

    (page_ordering_rules, pages)
}
pub fn are_pages_in_correct_order(
    pages: &[u32],
    page_ordering_rules: &[PageOrderingRules],
) -> bool {
    for (position, page) in pages.iter().enumerate() {
        let page_order = filter_page_rules(*page, page_ordering_rules);

        for (_pos, page_to_check) in pages.iter().enumerate().filter(|(pos, _)| *pos < position) {
            if page_order.contains(page_to_check) {
                return false;
            }
        }
    }

    true
}

pub fn filter_page_rules(page_no: u32, page_ordering_rules: &[PageOrderingRules]) -> Vec<u32> {
    page_ordering_rules
        .iter()
        .filter(|page_ordering_rule| page_ordering_rule.page_number_before == page_no)
        .map(|page_ordering_rule: &PageOrderingRules| page_ordering_rule.page_number)
        .collect()
}
