use crate::advent::day1::*;

pub fn run() {
    let (lhs, rhs) = read_input();

    let result = lhs
        .iter()
        .zip(rhs.clone())
        .fold(0, |acc, (lhs, rhs)| acc + (lhs).abs_diff(rhs));

    println!("{RESULT_TEXT_PREFIX} Part one: {result}");
}
