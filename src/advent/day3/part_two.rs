use crate::advent::day3::*;
use aho_corasick::{AhoCorasick, MatchKind};

pub fn run() {
    // must be this order for the AhoCorasick novel algorithm for non-overlapping matches
    let patterns = &["don't", "do"];

    let ac = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(patterns)
        .expect("ill-formed aho-corasick patterns");

    let mut calculate = true;
    let mut last_index = 0usize;

    let mut result = 0;
    let input = read_input();
    for mat in ac.find_iter(&input) {
        if calculate {
            result += add_all_muls_in(&input[last_index..mat.start()]);
        }

        match mat.pattern().as_i32() {
            // do
            1 => {
                calculate = true;
                last_index = mat.end()
            }
            // don't
            0 => calculate = false,
            _ => panic!("unexpected pattern ID"),
        };
    }

    if calculate {
        result += add_all_muls_in(&input[last_index..]);
    }

    println!("{RESULT_TEXT_PREFIX} Part two: {result}")
}
