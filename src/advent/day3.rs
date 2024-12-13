use crate::common;
use regex::Regex;
use std::path::Path;

pub mod part_one;
pub mod part_two;

const INPUT_LOCATION: &str = "input/3.txt";
const RESULT_TEXT_PREFIX: &str = "Day Three";

// can't be arsed to figure out a streaming solution
fn read_input() -> String {
    let input_location = Path::new(INPUT_LOCATION);

    if let Ok(file_data) = common::read_whole_file(input_location) {
        file_data
    } else {
        panic!("Failed to open input file {input_location:?}");
    }
}

fn add_all_muls_in(input: &str) -> u32 {
    let find_muls_regex = Regex::new(r"mul\((?P<lhs>\d{1,3}),(?P<rhs>\d{1,3})\)")
        .expect("Regex pattern was ill-formed");

    find_muls_regex
        .captures_iter(input)
        .map(|cs| cs.extract())
        .fold(0, |acc, (_, [lhs, rhs])| {
            acc + lhs
                .parse::<u32>()
                .expect("ill-formed regex rhs capture result")
                * rhs
                    .parse::<u32>()
                    .expect("ill-formed regex rhs capture result")
        })
}
