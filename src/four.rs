use crate::util::read_lines;

pub fn four() {
    let read_lines = read_lines("data/day-4/input.txt");
    let x_pos = find_all(&read_lines, 'X');

    let mut count = 0;
    for loc in x_pos {
        let count_xmas = count_xmas(loc, &read_lines);

        count += count_xmas;
    }

    println!("{}", count);

    let a_pos = find_all(&read_lines, 'A');

    let mut count = 0;
    for pos in a_pos {
        let is_x = check_x(pos, &read_lines);
        if is_x {
            count += 1;
        }
    }
    println!("{}", count);
}

fn check_x(a_pos: Location, lines: &Vec<String>) -> bool {
    let Location {
        vec_number,
        pos_in_string,
    } = a_pos;

    let ms = ['M', 'S'];

    let top_left = Location {
        vec_number: vec_number - 1,
        pos_in_string: pos_in_string - 1,
    };
    let check_top_left = get_letter(top_left, &lines);

    let bottom_right = Location {
        vec_number: vec_number + 1,
        pos_in_string: pos_in_string + 1,
    };
    let check_bottom_right = get_letter(bottom_right, &lines);

    if check_top_left.is_none() || check_bottom_right.is_none() {
        return false;
    }

    let tl = check_top_left.unwrap();
    let br = check_bottom_right.unwrap();
    if !(ms.contains(&tl) && ms.contains(&br) && tl != br) {
        return false;
    }

    let top_right = Location {
        vec_number: vec_number - 1,
        pos_in_string: pos_in_string + 1,
    };
    let check_top_right = get_letter(top_right, &lines);

    let bottom_left = Location {
        vec_number: vec_number + 1,
        pos_in_string: pos_in_string - 1,
    };
    let check_bottom_left = get_letter(bottom_left, &lines);

    if check_top_right.is_none() || check_bottom_left.is_none() {
        return false;
    }

    let tr = check_top_right.unwrap();
    let bl = check_bottom_left.unwrap();
    ms.contains(&tr) && ms.contains(&bl) && tr != bl
}

fn get_letter(a_pos: Location, lines: &Vec<String>) -> Option<char> {
    let Location {
        vec_number,
        pos_in_string,
    } = a_pos;

    if vec_number < 0 || pos_in_string < 0 {
        return None;
    }

    lines
        .get(vec_number as usize)
        .and_then(|x| x.chars().nth(pos_in_string as usize))
}

fn find_all(lines: &Vec<String>, searching_for: char) -> Vec<Location> {
    let mut pos = Vec::new();

    for (vec_number, line) in lines.iter().enumerate() {
        for (pos_in_string, letter) in line.chars().enumerate() {
            if letter == searching_for {
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
