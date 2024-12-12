use crate::advent::day2::*;

pub fn run() {
    let input = read_input();

    fn judge_safety_allow_single_removal(levels: &Vec<u32>) -> bool {
        if judge_safety_for_all(levels) {
            return true;
        }

        for idx in 0..levels.len() {
            // take away the indexed level from the levels
            let mut levels_minus_one: Vec<u32> = levels.iter().take(idx).cloned().collect();
            levels_minus_one.append(&mut levels.iter().skip(idx + 1).cloned().collect());

            if judge_safety_for_all(&levels_minus_one) {
                return true;
            }
        }

        false
    }

    let num_safe = input
        .iter()
        .filter(|levels| judge_safety_allow_single_removal(levels))
        .count();

    println!("{RESULT_TEXT_PREFIX} Part one: {num_safe}")
}
