use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashSet;

type InputType = Vec<String>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> InputType {
    input.lines().map(str::to_string).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &InputType) -> usize {
    let total: usize = data.iter().map(String::len).sum();
    let after: usize = data
        .iter()
        .map(|s| {
            let mut i = 1;
            let mut c = 0;
            while i < s.len() {
                if &s[i..i + 1] == "\\" {
                    if &s[i + 1..i + 2] == "x" {
                        i += 3;
                    } else {
                        i += 1;
                    }
                }
                c += 1;
                i += 1;
            }
            c - 1
        })
        .sum();
    total - after
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &InputType) -> usize {
    let before: usize = data.iter().map(String::len).sum();
    let after: usize = data
        .iter()
        .map(|s| {
            let mut i = 0;
            let mut c = 2;
            while i < s.len() {
                if &s[i..i + 1] == "\\" || &s[i..i + 1] == "\"" {
                    c += 1;
                }
                c += 1;
                i += 1;
            }
            c
        })
        .sum();
    after - before
}
