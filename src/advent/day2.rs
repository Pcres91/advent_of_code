use std::path::Path;

use crate::common;

pub mod part_one;
pub mod part_two;

const INPUT_LOCATION: &str = "input/2.txt";
const RESULT_TEXT_PREFIX: &str = "Day Two";

fn read_input() -> Vec<Vec<u32>> {
    let input_location = Path::new(INPUT_LOCATION);

    let mut input: Vec<Vec<u32>> = Vec::new();
    if let Ok(lines) = common::read_lines(input_location) {
        for line in lines.flatten() {
            let inputs: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().expect("Not an integer!"))
                .collect();

            input.push(inputs);
        }
    } else {
        panic!("Failed to open input file {input_location:?}");
    }

    input
}

fn judge_safety_for_all(levels: &Vec<u32>) -> bool {
    fn is_increasing(previous_level: u32, current_level: u32) -> bool {
        current_level > previous_level
    }

    fn judge_safety(previous_level: u32, current_level: u32, was_increasing: bool) -> bool {
        if is_increasing(previous_level, current_level) != was_increasing {
            return false;
        }

        let diff = current_level.abs_diff(previous_level);

        diff != 0 && diff <= 3
    }

    if levels.len() < 2 {
        return true;
    }

    let mut previous_level = levels[0];
    let increasing = is_increasing(previous_level, levels[1]);

    for current_level in levels.iter().skip(1) {
        if !judge_safety(previous_level, *current_level, increasing) {
            return false;
        }
        previous_level = *current_level;
    }

    true
}
