use std::iter::zip;

use crate::util::read_lines;

pub fn two() {
    let read_lines = read_lines("data/day-2/input.txt");
    let reports: Vec<Vec<i32>> = read_lines
        .iter()
        .map(|x| x.split(" ").map(|level| level.parse().unwrap()).collect())
        .collect();

    let mut num_correct = 0;

    for report in reports.iter() {
        let first = report[0];
        let second = report[1];

        let dir: Direction;
        if first > second {
            dir = Direction::Decreasing
        } else if first < second {
            dir = Direction::Increasing
        } else {
            continue;
        }

        if !check_okay(&first, &second, &dir) {
            continue;
        }

        let mut okay = true;
        for (first, second) in zip(&report[1..&report.len() - 1], &report[2..]) {
            okay = check_okay(first, second, &dir);
            if !okay {
                break;
            }
        }

        if okay {
            num_correct += 1;
        }
    }

    println!("{}", num_correct)
}

enum Direction {
    Increasing,
    Decreasing,
}

fn check_okay(first_num: &i32, second_num: &i32, direction: &Direction) -> bool {
    let direction_honored = match direction {
        Direction::Increasing => first_num < second_num,
        Direction::Decreasing => first_num > second_num,
    };

    if !direction_honored {
        return false;
    }

    let distance = (first_num - second_num).abs();

    distance <= 3
}
