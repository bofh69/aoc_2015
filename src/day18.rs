use advent_of_tools::*;
use aoc_runner_derive::{aoc, aoc_generator};
// use ahash::{HashMap, HashMapExt};
// use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[aoc(day18, part1)]
pub fn solve_part1(map: &Map) -> usize {
    let mut map = map.clone();

    for _ in 0..100 {
        map.transform(|map, p, c| {
            let n_neighbors = map.neighbors(p).filter(|(_p, _d, c)| *c == b'#').count();
            match (c, n_neighbors) {
                (b'#', 2 | 3) => b'#',
                (b'.', 3) => b'#',
                _ => b'.',
            }
        });
    }

    map.print();

    map.find(b'#').len()
}

#[aoc(day18, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let mut map = map.clone();

    for _ in 0..100 {
        map.transform(|map, p, c| {
            let right = map.get_width() - 1;
            let bottom = map.get_height() - 1;
            if (p.x == 0 || p.x == right) && (p.y == 0 || p.y == bottom) {
                return b'#';
            }
            let n_neighbors = map.neighbors(p).filter(|(_p, _d, c)| *c == b'#').count();
            match (c, n_neighbors) {
                (b'#', 2 | 3) => b'#',
                (b'.', 3) => b'#',
                _ => b'.',
            }
        });
    }

    map.print();

    map.find(b'#').len()
}
