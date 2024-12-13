use crate::advent::day5::*;

pub fn run() {
    let (rules, updates) = read_input();

    let mut valid_updates_sum = 0;
    for update in updates {
        if !is_valid_update(&update, &rules) {
            let valid_update = reorder_invalid_update(&update, &rules);
            valid_updates_sum += get_middle_number(&valid_update);
        }
    }

    println!("{RESULT_TEXT_PREFIX} Part two: {valid_updates_sum}")
}
