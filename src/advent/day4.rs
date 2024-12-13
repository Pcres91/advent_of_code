use crate::common;
use std::path::Path;

pub mod part_one;
pub mod part_two;

static INPUT_LOCATION: &'static str = "input/4.txt";
static RESULT_TEXT_PREFIX: &'static str = "Day Four";

static X: u8 = b'X';
static M: u8 = b'M';
static A: u8 = b'A';
static S: u8 = b'S';
static DOT: u8 = b'.'; // dummy char

static XMAS_PATTERN: [u8; 4] = [X, M, A, S];

fn read_input() -> Vec<Vec<u8>> {
    let input_location = Path::new(INPUT_LOCATION);

    let mut out = Vec::new();
    if let Ok(lines) = common::read_lines(input_location) {
        for line in lines.flatten() {
            out.push(line.as_bytes().into());
        }
    } else {
        panic!("Failed to open input file {input_location:?}");
    }

    out
}

fn xmas_found(arr_in: &[u8; 4]) -> bool {
    *arr_in == XMAS_PATTERN
}

// matrix rotation (transposition) of
// M.S
// .A.
// M.S
static X_MAS_PATTERN_1: [u8; 9] = [M, DOT, S, DOT, A, DOT, M, DOT, S];
static X_MAS_PATTERN_2: [u8; 9] = [M, DOT, M, DOT, A, DOT, S, DOT, S];
static X_MAS_PATTERN_3: [u8; 9] = [S, DOT, M, DOT, A, DOT, S, DOT, M];
static X_MAS_PATTERN_4: [u8; 9] = [S, DOT, S, DOT, A, DOT, M, DOT, M];
fn x_mas_found(arr_in: &[u8; 9]) -> bool {
    *arr_in == X_MAS_PATTERN_1
        || *arr_in == X_MAS_PATTERN_2
        || *arr_in == X_MAS_PATTERN_3
        || *arr_in == X_MAS_PATTERN_4
}
