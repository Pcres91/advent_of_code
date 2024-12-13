use crate::advent::day6::*;

fn set_position_as_patrolled(map: &mut Vec<Vec<u8>>, patroller: &Patroller) {
    if is_an_obstacle(map, &patroller.current_position) {
        let pos = &patroller.current_position;
        panic!("somehow trying to set an obstacle as patrolled. Position: {pos:?}")
    }

    map[patroller.current_position.y][patroller.current_position.x] = PATROLLED
}

pub fn run() {
    let (mut map, mut patroller) = read_input();

    set_position_as_patrolled(&mut map, &patroller);

    while patroller.move_next(&map) {
        set_position_as_patrolled(&mut map, &patroller);
    }

    let num_patrolled_blocks = map.iter().flatten().fold(0, |acc, c| {
        acc + match *c == PATROLLED {
            true => 1,
            false => 0,
        }
    });

    println!("{RESULT_TEXT_PREFIX} part one: {num_patrolled_blocks}")
}
