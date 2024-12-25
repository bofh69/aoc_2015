use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::min;

pub struct Data {
    l: u32,
    w: u32,
    h: u32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split('x').map(|s| s.parse().unwrap());
            Data {
                l: iter.next().unwrap(),
                w: iter.next().unwrap(),
                h: iter.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[Data]) -> u32 {
    data.iter()
        .map(|Data { l, w, h }| {
            let a = l * w;
            let b = w * h;
            let c = h * l;
            let m = min(min(a, b), c);
            2 * a + 2 * b + 2 * c + m
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[Data]) -> u32 {
    data.iter()
        .map(|Data { l, w, h }| {
            let a = 2 * (l + w);
            let b = 2 * (w + h);
            let c = 2 * (h + l);
            let vol = l * w * h;
            let m = min(min(a, b), c);
            vol + m
        })
        .sum()
}
