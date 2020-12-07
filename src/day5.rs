use aoc_runner_derive::{aoc, aoc_generator};
// use std::collections::HashSet;

type Data = String;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input.lines().map(|s| s.into()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    data.iter()
        .filter(|row| !row.contains("ab"))
        .filter(|row| !row.contains("cd"))
        .filter(|row| !row.contains("pq"))
        .filter(|row| !row.contains("xy"))
        .filter(|row| {
            row.chars()
                .fold((false, '-'), |(ok, last), c| ((ok || c == last), c))
                .0
        })
        .filter(|row| row.chars().filter(|&c| "aeiou".contains(c)).count() >= 3)
        .count()
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    data.iter()
        .filter(|row| {
            for i in 0..=row.len() - 2 {
                let cur = &row[i..i + 2];
                if row[i + 2..].contains(cur) {
                    return true;
                }
            }
            false
        })
        .filter(|row| {
            for i in 0..=row.len() - 3 {
                if row[i..=i] == row[i + 2..=i + 2] {
                    return true;
                }
            }
            false
        })
        .count()
}
