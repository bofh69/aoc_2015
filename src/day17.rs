// use ahash::{HashMap, HashMapExt};
// use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

type NumType = u8;
type InputType = Vec<NumType>;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> InputType {
    // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
    input
        .lines()
        .map(|s| NumType::from_str(s).unwrap())
        .collect()
}

/*
fn how_many(
    mem: &mut HashMap<(NumType, u32), HashSet<u32>>,
    left: NumType,
    used: u32,
    containers: &[NumType],
) -> HashSet<u32> {
    if let Some(result) = mem.get(&(left, used)) {
        return result.clone();
    }

    let mut result = HashSet::new();
    for (idx, &container) in containers.iter().enumerate() {
        if used & (1 << idx) != 0 {
            continue;
        }
        let new_used = used | (1 << idx);
        if container == left {
            result.insert(new_used);
            println!("{new_used}");
        } else if container < left {
            let res = how_many(mem, left - container, new_used, containers);
            mem.insert((left - container, new_used), res.clone());
            result = result.union(&res).cloned().collect();
        }
    }
    mem.insert((left, used), result.clone());

    result
}
*/

#[aoc(day17, part1)]
pub fn solve_part1(data: &InputType) -> u32 {
    // > 102
    // let mut mem = HashMap::new();
    // let _ = how_many(&mut mem, 150, 0, data).len() as NumType;

    let mut result = 0;
    for used in 1..=(1 << data.len()) {
        let sum: u32 = data
            .iter()
            .enumerate()
            .filter(|(i, _)| (1 << i) & used != 0)
            .map(|(_, v)| *v as u32)
            .sum();
        if sum == 150 {
            result += 1;
        }
    }
    result
}

#[aoc(day17, part2)]
pub fn solve_part2(data: &InputType) -> NumType {
    let mut fewest = u32::MAX;
    for used in 1u32..=(1 << data.len()) {
        let sum: u32 = data
            .iter()
            .enumerate()
            .filter(|(i, _)| (1 << i) & used != 0)
            .map(|(_, v)| *v as u32)
            .sum();
        if sum == 150 {
            let n_containers = used.count_ones();
            fewest = fewest.min(n_containers);
        }
    }
    println!("Fewest: {fewest}");
    let mut result = 0;
    for used in 1u32..=(1 << data.len()) {
        let n_containers = used.count_ones();
        if n_containers != fewest {
            continue;
        }
        let sum: u32 = data
            .iter()
            .enumerate()
            .filter(|(i, _)| (1 << i) & used != 0)
            .map(|(_, v)| *v as u32)
            .sum();
        if sum == 150 {
            result += 1;
        }
    }
    result
}
