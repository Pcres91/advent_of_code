use std::path::Path;

use crate::common;

pub mod part_one;
// pub mod part_two;

static INPUT_LOCATION: &'static str = "input/6.txt";
static RESULT_TEXT_PREFIX: &'static str = "Day Six";

static PATROLLED: u8 = b'X';
static OBSTACLE: u8 = b'#';
static STARTING_POSITION: char = '^';

fn read_input() -> (Vec<Vec<u8>>, Patroller) {
    let input_location = Path::new(INPUT_LOCATION);

    let mut out = Vec::new();
    let mut y_position = 0;
    let mut x_position = 0;
    let mut starting_position_found = false;
    if let Ok(lines) = common::read_lines(input_location) {
        for line in lines.flatten() {
            if !starting_position_found {
                match line.find(STARTING_POSITION as char) {
                    Some(idx) => {
                        starting_position_found = true;
                        x_position = idx;
                    }
                    None => y_position += 1,
                }
            }
            out.push(line.as_bytes().into());
        }
    } else {
        panic!("Failed to open input file {input_location:?}");
    }

    (out, Patroller::new(x_position, y_position))
}

fn is_an_obstacle(map: &Vec<Vec<u8>>, position: &Position) -> bool {
    map[position.y][position.x] == OBSTACLE
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

struct Patroller {
    current_position: Position,
    current_direction: Direction,
}

impl Patroller {
    pub fn new(starting_x: usize, starting_y: usize) -> Self {
        Patroller {
            current_position: Position {
                x: starting_x,
                y: starting_y,
            },
            current_direction: Direction::Up,
        }
    }

    fn swivel(&mut self) {
        match self.current_direction {
            Direction::Down => self.current_direction = Direction::Left,
            Direction::Left => self.current_direction = Direction::Up,
            Direction::Up => self.current_direction = Direction::Right,
            Direction::Right => self.current_direction = Direction::Down,
        }
    }

    fn get_next_position(&self, map: &Vec<Vec<u8>>) -> Option<Position> {
        match self.current_direction {
            Direction::Down => match self.current_position.y == map.len() - 1 {
                true => None,
                false => Some(Position {
                    x: self.current_position.x,
                    y: self.current_position.y + 1,
                }),
            },
            Direction::Left => match self.current_position.x == 0 {
                true => None,
                false => Some(Position {
                    x: self.current_position.x - 1,
                    y: self.current_position.y,
                }),
            },
            Direction::Up => match self.current_position.y == 0 {
                true => None,
                false => Some(Position {
                    x: self.current_position.x,
                    y: self.current_position.y - 1,
                }),
            },
            Direction::Right => match self.current_position.x == map[0].len() - 1 {
                true => None,
                false => Some(Position {
                    x: self.current_position.x + 1,
                    y: self.current_position.y,
                }),
            },
        }
    }

    pub fn move_next(&mut self, map: &Vec<Vec<u8>>) -> bool {
        let mut attempts = 0;

        while attempts < 4 {
            match self.get_next_position(map) {
                None => return false,
                Some(next_pos) => match is_an_obstacle(map, &next_pos) {
                    true => {
                        self.swivel();
                        attempts += 1
                    }
                    false => {
                        self.current_position = next_pos;
                        return true;
                    }
                },
            }
        }

        panic!("how did we get here?")
    }
}
