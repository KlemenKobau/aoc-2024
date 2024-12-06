use std::{collections::HashMap, iter::zip};

use crate::util::read_lines;

pub fn one() {
    let read_lines = read_lines("data/day-1/input.txt");
    let mut parsed_tuples: (Vec<i32>, Vec<i32>) = read_lines
        .iter()
        .map(|x| x.split_once("   "))
        .map(|x| x.unwrap())
        .map(|(x1, x2)| (x1.parse().unwrap(), x2.parse().unwrap()))
        .fold(
            (Vec::new(), Vec::new()),
            |(mut left, mut right), (x1, x2)| {
                left.push(x1);
                right.push(x2);
                (left, right)
            },
        );

    parsed_tuples.0.sort();
    parsed_tuples.1.sort();

    let sum_of_diff: i32 = zip(&parsed_tuples.0, &parsed_tuples.1)
        .into_iter()
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum();

    println!("{}", sum_of_diff);

    let (left, right) = parsed_tuples;

    let frequencies = right.iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let similarity_score: i32 = left
        .iter()
        .map(|x| frequencies.get(x).map_or(0, |frequency| frequency * x))
        .sum();

    println!("{}", similarity_score);
}
