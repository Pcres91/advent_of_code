use crate::advent::day4::*;

pub fn run() {
    let input = read_input();

    let mut xmas_count = 0;

    let x_len = input[0].len() as i64;
    let y_len = input.len() as i64;

    for x in 1..(x_len - 1) {
        for y in 1..(y_len - 1) {
            let is_valid_starting_point = input[y as usize][x as usize] == A;

            if is_valid_starting_point {
                let c = [
                    input[y as usize - 1][x as usize - 1],
                    DOT,
                    input[y as usize - 1][x as usize + 1],
                    DOT,
                    A,
                    DOT,
                    input[y as usize + 1][x as usize - 1],
                    DOT,
                    input[y as usize + 1][x as usize + 1],
                ];
                if x_mas_found(&c) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("{RESULT_TEXT_PREFIX} Part two: {xmas_count}")
}
