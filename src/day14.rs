// use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::*;
use std::str::FromStr;

type NumType = i32;
type InputType = Vec<(String, NumType, NumType, NumType)>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> InputType {
    // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
    input
        .lines()
        .map(|s| {
            let columns: Vec<_> = s.split(' ').collect();
            (
                columns[0].to_string(),
                NumType::from_str(columns[3]).unwrap(),
                NumType::from_str(columns[6]).unwrap(),
                NumType::from_str(columns[13]).unwrap(),
            )
        })
        .collect()
}

fn best_distance(timeout: NumType, data: &InputType) -> NumType {
    let mut best_length = 0;
    for (reeinder, speed, length, rest) in data {
        let whole = timeout / (length + rest);
        let rest_time = timeout % (length + rest);
        let mut total_length = whole * speed * length;
        if rest_time > *length {
            total_length += speed * length;
        } else {
            total_length += speed * rest_time;
        }
        if total_length > best_length {
            println!("New best: {reeinder}, {total_length} km");
            best_length = total_length;
        }
    }
    best_length
}

#[aoc(day14, part1)]
pub fn solve_part1(data: &InputType) -> NumType {
    best_distance(2503, data)
}

fn best_point(timeout: NumType, data: &InputType) -> NumType {
    let mut points = Vec::new();
    let mut lengths = Vec::new();
    points.resize(data.len(), 0);
    lengths.resize(data.len(), 0);

    for current_time in 0..timeout {
        for (i, (_reeinder, speed, length, rest)) in data.iter().enumerate() {
            let time = current_time % (length + rest);
            if time < *length {
                lengths[i] += speed;
            }
        }
        let max_length = lengths.iter().max().unwrap();
        for (i, length) in lengths.iter().enumerate() {
            if length == max_length {
                // println!("Point to {i}, length={length}");
                points[i] += 1;
            }
        }
    }

    *points.iter().max().unwrap()
}

#[aoc(day14, part2)]
pub fn solve_part2(data: &InputType) -> NumType {
    best_point(2503, data)
}
