use aoc_runner_derive::aoc;
// use std::collections::HashSet;

#[aoc(day4, part1)]
pub fn solve_part1(_data: &str) -> usize {
    for i in 0..(1 << 31) {
        let digest = md5::compute(format!("yzbqklnj{}", i));
        if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] & 0xf0 == 0 {
            dbg!(digest);
            return i;
        }
    }
    panic!("No solution found!");
}

#[aoc(day4, part2)]
pub fn solve_part2(_data: &str) -> usize {
    for i in 0..(1 << 31) {
        let digest = md5::compute(format!("yzbqklnj{}", i));
        if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0 {
            dbg!(digest);
            return i;
        }
    }
    panic!("No solution found!");
}
