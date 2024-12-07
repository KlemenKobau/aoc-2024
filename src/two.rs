use std::iter::zip;

use crate::util::read_lines;

pub fn two() {
    let read_lines = read_lines("data/day-2/input.txt");
    let mut reports: Vec<Vec<i32>> = read_lines
        .iter()
        .map(|x| x.split(" ").map(|level| level.parse().unwrap()).collect())
        .collect();

    let mut num_correct = 0;

    for report in reports.iter_mut() {
        let is_correct = check_correct(report);

        if is_correct {
            num_correct += 1;
            continue;
        }

        for number_to_exclude in 0..report.len() {
            let removed = report.remove(number_to_exclude);

            let is_correct = check_correct(report);

            if is_correct {
                num_correct += 1;
                break;
            } else {
                report.insert(number_to_exclude, removed);
            }
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

fn check_correct(report: &Vec<i32>) -> bool {
    let first = report[0];
    let second = report[1];

    let dir: Direction;
    if first > second {
        dir = Direction::Decreasing
    } else if first < second {
        dir = Direction::Increasing
    } else {
        return false;
    }

    if !check_okay(&first, &second, &dir) {
        return false;
    }

    let mut okay = true;
    for (first, second) in zip(&report[1..&report.len() - 1], &report[2..]) {
        okay = check_okay(first, second, &dir);
        if !okay {
            break;
        }
    }

    return okay;
}
