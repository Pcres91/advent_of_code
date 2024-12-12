use crate::advent::day3::*;

pub fn run() {
    let res = add_all_muls_in(&read_input());

    println!("{RESULT_TEXT_PREFIX} Part one: {res}")
}
