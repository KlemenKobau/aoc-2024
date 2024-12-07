use crate::util::read_lines;

pub fn four() {
    let read_lines = read_lines("data/day-4/input.txt");
    let x_pos = find_all_x(&read_lines);

    let mut count = 0;
    for loc in x_pos {
        let count_xmas = count_xmas(loc, &read_lines);

        count += count_xmas;
    }

    println!("{}", count);
}

fn find_all_x(lines: &Vec<String>) -> Vec<Location> {
    let mut pos = Vec::new();

    for (vec_number, line) in lines.iter().enumerate() {
        for (pos_in_string, letter) in line.chars().enumerate() {
            if letter == 'X' {
                pos.push(Location {
                    vec_number: vec_number as i32,
                    pos_in_string: pos_in_string as i32,
                });
            }
        }
    }

    pos
}

fn count_xmas(start_location: Location, lines: &Vec<String>) -> i32 {
    let letters_to_check = ['X', 'M', 'A', 'S'];

    let mut num_found = 0;

    let mut deltas = Vec::new();

    // check top down
    let delta = (-1, 0);
    deltas.push(delta);

    // check bottom up
    let delta = (1, 0);
    deltas.push(delta);

    // check left right
    let delta = (0, 1);
    deltas.push(delta);

    // check right left
    let delta = (0, -1);
    deltas.push(delta);

    // check diag upper left
    let delta = (-1, -1);
    deltas.push(delta);

    // check diag upper right
    let delta = (-1, 1);
    deltas.push(delta);

    // check diag bottom left
    let delta = (1, -1);
    deltas.push(delta);

    // check diag bottom right
    let delta = (1, 1);
    deltas.push(delta);

    for delta in deltas {
        let found = iter(start_location.clone(), lines, delta, letters_to_check);
        if found {
            num_found += 1;
        }
    }

    num_found
}

fn iter(
    start_location: Location,
    lines: &Vec<String>,
    delta: (i32, i32),
    letters_to_check: [char; 4],
) -> bool {
    let Location {
        mut vec_number,
        mut pos_in_string,
    } = start_location;

    for letter in letters_to_check {
        let correct = check_location(vec_number, pos_in_string, letter, lines);
        if !correct {
            return false;
        }

        vec_number += delta.0;
        pos_in_string += delta.1;
    }

    return true;
}

fn check_location(
    vec_number: i32,
    pos_in_string: i32,
    letter_to_check: char,
    lines: &Vec<String>,
) -> bool {
    if vec_number < 0 || pos_in_string < 0 {
        return false;
    }

    lines
        .get(vec_number as usize)
        .and_then(|x| x.chars().nth(pos_in_string as usize))
        .map(|x| x == letter_to_check)
        .unwrap_or(false)
}

#[derive(Clone)]
struct Location {
    vec_number: i32,
    pos_in_string: i32,
}
