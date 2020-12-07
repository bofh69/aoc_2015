use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub enum Data {
    On(usize, usize, usize, usize),
    Off(usize, usize, usize, usize),
    Toggle(usize, usize, usize, usize),
}

fn parse_coord(s: &str) -> (usize, usize) {
    let mut iter = s.split(',').map(|s| s.parse().unwrap());

    (iter.next().unwrap(), iter.next().unwrap())
}

// parse "424,675 through 740,862"
fn parse(s: &str) -> (usize, usize, usize, usize) {
    let mut iter = s.split(' ');

    let (x1, y1) = parse_coord(iter.next().unwrap());
    iter.next();
    let (x2, y2) = parse_coord(iter.next().unwrap());
    (x1, y1, x2, y2)
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|s| {
            if s.starts_with("toggle") {
                // toggle 424,675 through 740,862
                let (x1, y1, x2, y2) = parse(&s[7..]);
                Data::Toggle(x1, y1, x2, y2)
            } else if s.starts_with("turn on") {
                // turn on 715,871 through 722,890
                let (x1, y1, x2, y2) = parse(&s[8..]);
                Data::On(x1, y1, x2, y2)
            } else if s.starts_with("turn off") {
                // turn off 446,432 through 458,648
                let (x1, y1, x2, y2) = parse(&s[9..]);
                Data::Off(x1, y1, x2, y2)
            } else {
                panic!("Unknown line");
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[Data]) -> usize {
    let mut grid = HashSet::new();
    data.iter().for_each(|inst| match inst {
        Data::On(x1, y1, x2, y2) => {
            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    grid.insert((x, y));
                }
            }
        }
        Data::Off(x1, y1, x2, y2) => {
            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    grid.remove(&(x, y));
                }
            }
        }
        Data::Toggle(x1, y1, x2, y2) => {
            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    if grid.contains(&(x, y)) {
                        grid.remove(&(x, y));
                    } else {
                        grid.insert((x, y));
                    }
                }
            }
        }
    });
    grid.len()
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &[Data]) -> usize {
    let mut grid = [0; 1000 * 1000];
    data.iter().for_each(|inst| match inst {
        Data::On(x1, y1, x2, y2) => {
            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    grid[x + y * 1000] += 1;
                }
            }
        }
        Data::Off(x1, y1, x2, y2) => {
            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    let idx = x + y * 1000;
                    if grid[idx] > 0 {
                        grid[idx] -= 1;
                    }
                }
            }
        }
        Data::Toggle(x1, y1, x2, y2) => {
            for x in *x1..=*x2 {
                for y in *y1..=*y2 {
                    grid[x + y * 1000] += 2;
                }
            }
        }
    });
    grid.iter().sum()
}
