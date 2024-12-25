use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Unkown char"),
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[i32]) -> usize {
    let mut acc = 0;
    for (position, val) in data.iter().enumerate() {
        acc += val;
        if acc == -1 {
            return position + 1;
        }
    }
    panic!("No solution found");
}
