use crate::advent::day6::*;

const UP: u8 = 0b0001;
const RIGHT: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const DOWN: u8 = 0b1000;

impl Patroller {
    pub fn can_move_forwards(&self, map: &Vec<Vec<u8>>) -> bool {
        match self.get_next_position_forward(map) {
            None => false,
            Some(next_pos_forward) => return !is_an_obstacle(map, &next_pos_forward),
        }
    }
}

fn current_position_patrolled_and_matches_direction(
    map: &Vec<Vec<u8>>,
    patroller: &Patroller,
) -> bool {
    let current_map_val = map[patroller.current_position.y][patroller.current_position.x];

    let mut patrolled = false;

    if current_map_val != UNPATROLLED {
        match patroller.current_direction {
            Direction::Down => patrolled = current_map_val & DOWN > 0,
            Direction::Right => patrolled = current_map_val & RIGHT > 0,
            Direction::Up => patrolled = current_map_val & UP > 0,
            Direction::Left => patrolled = current_map_val & LEFT > 0,
        }
    }

    patrolled
}

fn set_position_as_patrolled(map: &mut Vec<Vec<u8>>, patroller: &Patroller) {
    if is_an_obstacle(map, &patroller.current_position) {
        let pos = &patroller.current_position;
        panic!("somehow trying to set an obstacle as patrolled. Position: {pos:?}")
    }

    let mut map_value = map[patroller.current_position.y][patroller.current_position.x];

    let char_val = map_value as char;
    match map_value {
        PATROLLED => panic!("this is for part one."),
        UNPATROLLED => map[patroller.current_position.y][patroller.current_position.x] = 0,
        STARTING_POSITION => map[patroller.current_position.y][patroller.current_position.x] = 0,
        0..0b1111 => (),
        0b1111 => panic!("already patrolled in all directions."),
        0b1111.. => panic!("unhandled map type {char_val}"),
    };

    map_value = map[patroller.current_position.y][patroller.current_position.x];

    let current_direction = &patroller.current_direction;
    match current_direction {
        Direction::Down => {
            if map_value & DOWN > 0 {
                panic!("going {current_direction:?} but already been")
            }
            map[patroller.current_position.y][patroller.current_position.x] |= DOWN;
        }
        Direction::Right => {
            if map_value & RIGHT > 0 {
                panic!("going {current_direction:?} but already been")
            }
            map[patroller.current_position.y][patroller.current_position.x] |= RIGHT;
        }
        Direction::Up => {
            if map_value & UP > 0 {
                panic!("going {current_direction:?} but already been")
            }
            map[patroller.current_position.y][patroller.current_position.x] |= UP;
        }
        Direction::Left => {
            if map_value & LEFT > 0 {
                panic!("going {current_direction:?} but already been")
            }
            map[patroller.current_position.y][patroller.current_position.x] |= LEFT;
        }
    }
}

fn look_right_and_already_been_moving_in_that_direction(
    map: &Vec<Vec<u8>>,
    patroller: &Patroller,
) -> bool {
    let mut clone = patroller.clone();

    clone.swivel();

    while clone.can_move_forwards(map) {
        clone.move_next(map);
        if current_position_patrolled_and_matches_direction(map, &clone) {
            return true;
        }
    }

    false
}

pub fn run() {
    let (mut map, mut patroller) = read_input();

    let mut obstacles_to_set_to_create_loops: Vec<Position> = Vec::new();

    set_position_as_patrolled(&mut map, &patroller);

    while patroller.move_next(&map) {
        set_position_as_patrolled(&mut map, &patroller);
        if look_right_and_already_been_moving_in_that_direction(&map, &patroller) {
            if let Some(next_position) = patroller.get_next_position_forward(&map) {
                if is_an_obstacle(&map, &next_position) {
                    panic!("already moving in a loop. {patroller:?}.\n{map:#?}");
                } else {
                    obstacles_to_set_to_create_loops.push(next_position);
                }
            } else {
                panic!("Somehow looked right, saw patroller had been moving on the edge of the map. not possible, state gone bad.");
            }
        }
    }

    let num_loopable_positions = obstacles_to_set_to_create_loops.len();

    // println!("discovered: {obstacles_to_set_to_create_loops:#?}");

    println!("{RESULT_TEXT_PREFIX} part two: {num_loopable_positions}")
}
