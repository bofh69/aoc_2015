// use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::*;
// use std::str::FromStr;

type InputType = String;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> InputType {
    input.to_string()
}

fn look_and_see(data: &str) -> String {
    let mut result = String::new();
    let mut old_c = '@';
    let mut count = 0;
    for c in data.chars() {
        if c != old_c && count > 0 {
            result.push_str(&format!("{count}{old_c}"));
            count = 0;
        }
        count += 1;
        old_c = c;
    }
    result.push_str(&format!("{count}{old_c}"));
    result
}

#[aoc(day10, part1)]
pub fn solve_part1(data: &InputType) -> usize {
    let mut result = data.clone();
    for _i in 0..40 {
        result = look_and_see(&result);
    }
    result.len()
}

#[aoc(day10, part2)]
pub fn solve_part2(data: &InputType) -> usize {
    let mut result = data.clone();
    for _i in 0..50 {
        result = look_and_see(&result);
    }
    result.len()
}
