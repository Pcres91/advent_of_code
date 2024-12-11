use std::fs::File;
use std::io::{self, BufRead};
use std::panic;
use std::path::Path;

mod common;

fn day1() {
    println!("entered day 1");

    let input_location = Path::new("input/1.txt");

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    let mut lhs = vec![];
    let mut rhs = vec![];
    if let Ok(lines) = read_lines(input_location) {
        for line in lines.flatten() {
            // println!("{line}");
            let inputs: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().expect("Not an integer!"))
                .collect();
            let len = inputs.len();
            if (len != 2) {
                panic!("input was not the right size. Len: {len}");
            }
            lhs.push(inputs[0]);
            rhs.push(inputs[1]);
        }
    } else {
        panic!("Failed to open input file {input_location:?}");
    }

    lhs.sort();
    rhs.sort();

    let result = lhs
        .iter()
        .zip(rhs)
        .fold(0, |acc, (lhs, rhs)| acc + (lhs).abs_diff(rhs));

    println!("Part one: {result}")
}

fn main() {
    day1()
}
