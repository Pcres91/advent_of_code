use regex::Regex;

use crate::advent::day3::*;

pub fn run() {
    let find_muls_regex = Regex::new(r"mul\((?P<lhs>\d{1,3}),(?P<rhs>\d{1,3})\)")
        .expect("Regex pattern was ill-formed");

    let res = find_muls_regex
        .captures_iter(&read_input())
        .map(|cs| cs.extract())
        .fold(0, |acc, (_, [lhs, rhs])| {
            acc + lhs
                .parse::<u32>()
                .expect("ill-formed regex rhs capture result")
                * rhs
                    .parse::<u32>()
                    .expect("ill-formed regex rhs capture result")
        });

    println!("{RESULT_TEXT_PREFIX} Part one: {res}")
}
