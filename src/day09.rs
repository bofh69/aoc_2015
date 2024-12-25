use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::*;
use std::str::FromStr;

type NumType = u32;
type InputType = Vec<(String, String, NumType)>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> InputType {
    // Faerun to Norrath = 129
    input
        .lines()
        .map(|s| {
            let columns: Vec<_> = s.split(' ').collect();
            (
                columns[0].to_string(),
                columns[2].to_string(),
                NumType::from_str(columns[4]).unwrap(),
            )
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &InputType) -> NumType {
    let mut edges = HashMap::new();
    let mut places = Vec::new();
    for e in data {
        edges.insert((e.0.as_str(), e.1.as_str()), e.2);
        edges.insert((e.1.as_str(), e.0.as_str()), e.2);
        if !places.contains(&e.0.as_str()) {
            places.push(e.0.as_str());
        }
        if !places.contains(&e.1.as_str()) {
            places.push(e.1.as_str());
        }
    }

    let mut min_cost = NumType::MAX;
    for path in places.iter().permutations(places.len()) {
        let mut cost = 0;
        let mut old_place = "";
        for place in path {
            if let Some(c) = edges.get(&(old_place, place)) {
                cost += c;
            }
            old_place = place;
        }
        min_cost = min_cost.min(cost);
    }

    min_cost
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &InputType) -> NumType {
    let mut edges = HashMap::new();
    let mut places = Vec::new();
    for e in data {
        edges.insert((e.0.as_str(), e.1.as_str()), e.2);
        edges.insert((e.1.as_str(), e.0.as_str()), e.2);
        if !places.contains(&e.0.as_str()) {
            places.push(e.0.as_str());
        }
        if !places.contains(&e.1.as_str()) {
            places.push(e.1.as_str());
        }
    }

    let mut max_cost = 0;
    for path in places.iter().permutations(places.len()) {
        let mut cost = 0;
        let mut old_place = "";
        for place in path {
            if let Some(c) = edges.get(&(old_place, place)) {
                cost += c;
            }
            old_place = place;
        }
        max_cost = max_cost.max(cost);
    }

    max_cost
}
