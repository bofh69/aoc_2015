use aoc_runner_derive::{aoc, aoc_generator};
use regex::*;
use std::str::FromStr;

type InputType = String;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> InputType {
    input.to_string()
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &InputType) -> i32 {
    let re = Regex::new("(-?[0-9]+)").unwrap();
    let mut sum = 0;
    for m in re.captures_iter(data) {
        sum += i32::from_str(&m[0]).unwrap();
    }
    sum
}

fn get_sum(parsed: &json::JsonValue) -> i32 {
    use json::JsonValue::*;

    match parsed {
        Null => 0,
        Short(_) => 0,
        String(_) => 0,
        Number(n) => n.as_fixed_point_i64(0).unwrap() as i32,
        Boolean(_) => 0,
        Object(o) => {
            let mut sum = 0;
            let mut any_red = false;
            for (_, v) in o.iter() {
                if let Short(s) = v {
                    if s == "red" {
                        any_red = true;
                        break;
                    }
                } else {
                    sum += get_sum(v);
                }
            }
            if any_red {
                0
            } else {
                sum
            }
        }
        Array(arr) => {
            let mut sum = 0;
            for v in arr {
                sum += get_sum(v);
            }
            sum
        }
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &InputType) -> i32 {
    let parsed = json::parse(data).unwrap();

    get_sum(&parsed)
}
