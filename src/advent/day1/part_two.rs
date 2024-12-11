use crate::advent::day1::{read_input, RESULT_TEXT_PREFIX};

pub fn run() {
    let (lhs, rhs) = read_input();

    let part_2_result = lhs.iter().fold(0, |acc, val| {
        acc + (val * rhs.iter().filter(|x| *x == val).count() as u32)
    });

    println!("{RESULT_TEXT_PREFIX}: {part_2_result}");   
}