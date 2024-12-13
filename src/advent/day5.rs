use crate::common;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

pub mod part_one;
// pub mod part_two;

static INPUT_LOCATION: &'static str = "input/5.txt";
static RESULT_TEXT_PREFIX: &'static str = "Day Five";

#[derive(Debug)]
struct Rules {
    data: HashMap<u32, HashSet<u32>>,
}

impl Rules {
    pub fn new() -> Self {
        Rules {
            data: HashMap::new(),
        }
    }

    pub fn add_new_rule(&mut self, pages: Vec<u32>) {
        if pages.len() != 2 {
            panic!("Invalid new rule: {pages:?}");
        }
        let before = pages[0];
        let after = pages[1];

        if let Some(afters) = self.data.get_mut(&before) {
            afters.insert(after);
        } else {
            let mut new_data = HashSet::new();
            new_data.insert(after);
            self.data.insert(before, new_data);
        }
    }

    pub fn pages_after_are_valid(&self, page: u32, pages_after: &[u32]) -> bool {
        if pages_after.len() == 0 {
            return true;
        }
        if let Some(rule_pages_after) = self.data.get(&page) {
            !pages_after
                .iter()
                .any(|page| !rule_pages_after.contains(page))
        } else {
            false
        }
    }
}

fn read_input() -> (Rules, Vec<Vec<u32>>) {
    let input_location = Path::new(INPUT_LOCATION);

    let mut rules = Rules::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = common::read_lines(input_location) {
        for line in lines.flatten() {
            if line.contains('|') {
                rules.add_new_rule(
                    line.split('|')
                        .map(|s| s.parse::<u32>().expect("invalid string to parse as u32"))
                        .collect(),
                );
            } else if line.contains(',') {
                updates.push(
                    line.split(',')
                        .map(|s| s.parse::<u32>().expect("invalid string to parse as u32"))
                        .collect(),
                );
            }
        }
    } else {
        panic!("Failed to open input file {input_location:?}");
    }

    (rules, updates)
}

fn is_valid_update(update: &Vec<u32>, rules: &Rules) -> bool {
    fn page_is_valid(page_idx: usize, update: &[u32], rules: &Rules) -> bool {
        let page = update[page_idx];
        let pages_after = &update[page_idx + 1..];
        rules.pages_after_are_valid(page, pages_after)
    }

    for page_idx in 0..update.len() {
        if !page_is_valid(page_idx, update, rules) {
            return false;
        }
    }

    true
}

fn get_middle_number(rules: &[u32]) -> u32 {
    let middle_idx = rules.len() / 2;
    rules[middle_idx]
}
