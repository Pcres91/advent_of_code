use std::path::Path;

use crate::common;

pub mod part_one;
pub mod part_two;

static INPUT_LOCATION: &'static str = "input/1.txt";
static RESULT_TEXT_PREFIX: &'static str = "Day One";

fn read_input() -> (Vec<u32>, Vec<u32>) {
    let input_location = Path::new(INPUT_LOCATION);

    let mut lhs = vec![];
    let mut rhs = vec![];
    if let Ok(lines) = common::read_lines(input_location) {
        for line in lines.flatten() {
            // println!("{line}");
            let inputs: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().expect("Not an integer!"))
                .collect();
            let len = inputs.len();
            if len != 2 {
                panic!("input was not the right size. Len: {len}");
            }
            lhs.push(inputs[0]);
            rhs.push(inputs[1]);
        }
    } else {
        panic!("Failed to open input file {input_location:?}");
    }

    lhs.sort();
    rhs.sort();

    (lhs, rhs)
}
