use std::path::Path;

use crate::common;

pub mod part_one;
// pub mod part_two;

static INPUT_LOCATION: &'static str = "input/3.txt";
static RESULT_TEXT_PREFIX: &'static str = "Day Three";

fn read_input() -> String {
    let input_location = Path::new(INPUT_LOCATION);

    if let Ok(file_data) = common::read_whole_file(input_location) {
        file_data
    } else {
        panic!("Failed to open input file {input_location:?}");
    }
}
