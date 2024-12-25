use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::*;
use std::str::FromStr;

type NumType = i32;
type InputType = Vec<(String, String, NumType)>;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> InputType {
    // Alice would gain 54 happiness units by sitting next to Bob.
    input
        .lines()
        .map(|s| {
            let columns: Vec<_> = s.split(' ').collect();
            let who = columns[0].to_string();
            let who2 = columns[10];
            let who2 = who2[0..who2.len() - 1].to_string();
            if columns[2] == "gain" {
                (who, who2, NumType::from_str(columns[3]).unwrap())
            } else {
                (who, who2, -NumType::from_str(columns[3]).unwrap())
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &InputType) -> NumType {
    let mut edges = HashMap::new();
    let mut persons = Vec::new();
    for e in data {
        edges.insert((e.0.as_str(), e.1.as_str()), e.2);
        if !persons.contains(&e.0.as_str()) {
            persons.push(e.0.as_str());
        }
        if !persons.contains(&e.1.as_str()) {
            persons.push(e.1.as_str());
        }
    }

    let n_persons = persons.len();

    let mut max_happiness = NumType::MIN;
    for seating in persons.iter().permutations(persons.len()) {
        let mut happiness = 0;
        for i in 0..n_persons {
            let before = seating[(i + n_persons - 1) % n_persons];
            let person = seating[i];
            let after = seating[(i + 1) % n_persons];
            if let Some(c) = edges.get(&(person, after)) {
                happiness += c;
            } else {
                panic!("Unknown persons {person}-{after}");
            }
            if let Some(c) = edges.get(&(person, before)) {
                happiness += c;
            } else {
                panic!("Unknown persons {person}-{before}");
            }
        }
        max_happiness = max_happiness.max(happiness);
    }

    max_happiness
}

#[aoc(day13, part2)]
pub fn solve_part2(data: &InputType) -> NumType {
    let mut edges = HashMap::new();
    let mut persons = Vec::new();
    for e in data {
        edges.insert((e.0.as_str(), e.1.as_str()), e.2);
        if !persons.contains(&e.0.as_str()) {
            persons.push(e.0.as_str());
        }
        if !persons.contains(&e.1.as_str()) {
            persons.push(e.1.as_str());
        }
    }

    persons.push("me");
    let n_persons = persons.len();

    let mut max_happiness = NumType::MIN;
    for seating in persons.iter().permutations(persons.len()) {
        let mut happiness = 0;
        for i in 0..n_persons {
            let before = seating[(i + n_persons - 1) % n_persons];
            let person = seating[i];
            let after = seating[(i + 1) % n_persons];
            if let Some(c) = edges.get(&(person, after)) {
                happiness += c;
            }
            if let Some(c) = edges.get(&(person, before)) {
                happiness += c;
            }
        }
        max_happiness = max_happiness.max(happiness);
    }

    max_happiness
}
