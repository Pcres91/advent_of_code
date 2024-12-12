use crate::advent::day2::*;

pub fn run() {
    let input = read_input();

    let num_safe = input
        .iter()
        .filter(|levels| judge_safety_for_all(&levels))
        .count();

    println!("{RESULT_TEXT_PREFIX} Part one: {num_safe}")
}
