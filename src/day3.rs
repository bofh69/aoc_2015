use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub enum Dir {
    North,
    South,
    East,
    West,
}

type Data = Dir;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .chars()
        .map(|c| match c {
            '^' => Dir::North,
            'v' => Dir::South,
            '<' => Dir::West,
            '>' => Dir::East,
            _ => panic!("Unknown char"),
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    let mut visited = HashSet::new();
    let mut pos = (0, 0);
    visited.insert(pos);
    data.iter().for_each(|dir| {
        use Dir::*;
        match dir {
            North => pos = (pos.0, pos.1 - 1),
            South => pos = (pos.0, pos.1 + 1),
            East => pos = (pos.0 + 1, pos.1),
            West => pos = (pos.0 - 1, pos.1),
        }
        visited.insert(pos);
    });
    visited.len()
}

#[aoc(day3, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    let mut visited = HashSet::new();
    let mut pos = [(0, 0), (0, 0)];
    let mut who = 0;
    visited.insert(pos[who]);
    data.iter().for_each(|dir| {
        use Dir::*;
        match dir {
            North => pos[who] = (pos[who].0, pos[who].1 - 1),
            South => pos[who] = (pos[who].0, pos[who].1 + 1),
            East => pos[who] = (pos[who].0 + 1, pos[who].1),
            West => pos[who] = (pos[who].0 - 1, pos[who].1),
        }
        visited.insert(pos[who]);
        who = (who + 1) & 1;
    });
    visited.len()
}
