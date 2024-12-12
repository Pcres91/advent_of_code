use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_whole_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename)
}
