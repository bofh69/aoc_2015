// use ahash::{HashMap, HashMapExt};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

type NumType = i64;
type InputType = Vec<[NumType; 5]>;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> InputType {
    // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
    input
        .lines()
        .map(|s| {
            let columns: Vec<_> = s.split(' ').collect();
            let n1 = columns[2];
            let n1 = &n1[0..n1.len() - 1];
            let n2 = columns[4];
            let n2 = &n2[0..n2.len() - 1];
            let n3 = columns[6];
            let n3 = &n3[0..n3.len() - 1];
            let n4 = columns[8];
            let n4 = &n4[0..n4.len() - 1];
            let n5 = columns[10];
            [
                NumType::from_str(n1).unwrap(),
                NumType::from_str(n2).unwrap(),
                NumType::from_str(n3).unwrap(),
                NumType::from_str(n4).unwrap(),
                NumType::from_str(n5).unwrap(),
            ]
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &InputType) -> NumType {
    let mut best_sum = 0;

    let mut v = vec![0; data.len()];

    'try_comb: loop {
        let v_sum: NumType = v.iter().take(data.len() - 1).sum();
        v[data.len() - 1] = 100 - v_sum;

        if v[data.len() - 1] >= 0 {
            let mut sum = [0; 4];
            for (i, ing) in data.iter().enumerate() {
                sum[0] += ing[0] * v[i];
                sum[1] += ing[1] * v[i];
                sum[2] += ing[2] * v[i];
                sum[3] += ing[3] * v[i];
            }

            let ok = sum.iter().take(4).filter(|&&v| v < 0).count() == 0;
            if ok {
                let sum = sum[0] * sum[1] * sum[2] * sum[3];
                best_sum = best_sum.max(sum);
            }
        }
        for i in 0..data.len() - 1 {
            let i = data.len() - 2 - i;
            v[i] += 1;
            if v[i] > 100 {
                if i == 0 {
                    break 'try_comb;
                }
                v[i] = 0;
                continue;
            } else {
                break;
            }
        }
    }
    best_sum
}

#[aoc(day15, part2)]
pub fn solve_part2(data: &InputType) -> NumType {
    let mut best_sum = 0;

    let mut v = vec![0; data.len()];

    'try_comb: loop {
        let v_sum: NumType = v.iter().take(data.len() - 1).sum();
        v[data.len() - 1] = 100 - v_sum;

        if v[data.len() - 1] >= 0 {
            let mut sum = [0; 5];
            for (i, ing) in data.iter().enumerate() {
                sum[0] += ing[0] * v[i];
                sum[1] += ing[1] * v[i];
                sum[2] += ing[2] * v[i];
                sum[3] += ing[3] * v[i];
                sum[4] += ing[4] * v[i];
            }

            let ok = sum[4] == 500 && sum.iter().take(4).filter(|&&v| v < 0).count() == 0;
            if ok {
                let sum = sum[0] * sum[1] * sum[2] * sum[3];
                best_sum = best_sum.max(sum);
            }
        }
        for i in 0..data.len() - 1 {
            let i = data.len() - 2 - i;
            v[i] += 1;
            if v[i] > 100 {
                if i == 0 {
                    break 'try_comb;
                }
                v[i] = 0;
                continue;
            } else {
                break;
            }
        }
    }
    best_sum
}
