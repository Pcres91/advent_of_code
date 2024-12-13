use crate::advent::day4::*;

pub fn run() {
    let input = read_input();

    let mut xmas_count = 0;

    let x_len = input[0].len() as i64;
    let y_len = input.len() as i64;
    let xmas_len = XMAS_PATTERN.len() as i64;

    // could have done almost half as many checks if I'd checked for backwards-ness in xmas_found
    for x in 0..x_len {
        for y in 0..y_len {
            let eastward_valid = (x + xmas_len) <= x_len;
            let westward_valid = (x + 1 - xmas_len) >= 0;
            let northward_valid = (y + 1 - xmas_len) >= 0;
            let southward_valid = (y + xmas_len) <= y_len;
            // horizontal forwards
            if eastward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize][x as usize + 1],
                    input[y as usize][x as usize + 2],
                    input[y as usize][x as usize + 3],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // horizontal backwards
            if westward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize][x as usize - 1],
                    input[y as usize][x as usize - 2],
                    input[y as usize][x as usize - 3],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // vertical downwards
            if southward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize + 1][x as usize],
                    input[y as usize + 2][x as usize],
                    input[y as usize + 3][x as usize],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // vertical upwards
            if northward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize - 1][x as usize],
                    input[y as usize - 2][x as usize],
                    input[y as usize - 3][x as usize],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // diagonal NE
            if eastward_valid && northward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize - 1][x as usize + 1],
                    input[y as usize - 2][x as usize + 2],
                    input[y as usize - 3][x as usize + 3],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // diagonal SE
            if eastward_valid && southward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize + 1][x as usize + 1],
                    input[y as usize + 2][x as usize + 2],
                    input[y as usize + 3][x as usize + 3],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // diagonal NW
            if westward_valid && northward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize - 1][x as usize - 1],
                    input[y as usize - 2][x as usize - 2],
                    input[y as usize - 3][x as usize - 3],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
            // diagonal SW
            if westward_valid && southward_valid {
                let c = [
                    input[y as usize][x as usize],
                    input[y as usize + 1][x as usize - 1],
                    input[y as usize + 2][x as usize - 2],
                    input[y as usize + 3][x as usize - 3],
                ];
                if xmas_found(&c) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("{RESULT_TEXT_PREFIX} Part one: {xmas_count}")
}
