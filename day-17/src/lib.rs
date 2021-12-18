#![allow(warnings)]
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::str::FromStr;

type ProblemInput = ((u64, u64), (i64,i64));
fn from_range<T: FromStr>(c: &str) -> (T, T)  where <T as FromStr>::Err: Debug {
    let (_ignore, c) = c.split_once("=").unwrap();
    let (x_start, x_end) = c.split_once("..").unwrap();
    (x_start.parse().unwrap(), x_end.parse().unwrap())
}
fn parse(input: &str) -> ProblemInput {
    let (_ignore, coordinates) = input.trim().split_once("target area: ").unwrap();
    let (x_range, y_range) = coordinates.trim().split_once(",").unwrap();

    (from_range(x_range), from_range(y_range))
}
type Part1Output = u64;
fn part_1(input: ProblemInput) -> Part1Output {
    let min_y = input.1.0;
    let c = (min_y + 1).abs() as u64;
    c*(c+1)/2
}

type Part2Output = u64;
fn part_2(input: ProblemInput) -> Part2Output {
    let max_initial_x_speed = input.0.1;
    let min_initial_x_speed = find_minimum_x_speed(input.0.0);
    let min_initial_y_speed = input.1.0;
    let max_initial_y_speed = input.1.0.abs();
    let mut total_starting_speeds = 0;
    let mut c = 0;
    for x in min_initial_x_speed..=max_initial_x_speed {
        for y in min_initial_y_speed..=max_initial_y_speed {
            // dbg!((x,y));
            c+=1;
            if will_go((x,y), &input) {
                total_starting_speeds += 1;
            }
        }
    }
    dbg!(c);
    total_starting_speeds
}

fn will_go(candidate: (u64, i64), p: &ProblemInput) -> bool {
    // TODO this can be optimized for the cases of starting positive Y speed by taking into account
    // the fact that the point in which the position crosses the y=0 axis again, the Y speed will be the opposite of the starting Y speed
    let mut pos = (0,0);
    let mut speed = candidate;

    while pos.0 <= p.0.1 && pos.1 >= p.1.0 {
        if p.0.0 <= pos.0 && pos.0 <= p.0.1 &&
            p.1.0 <= pos.1 && pos.1 <= p.1.1 {
            return true
        }
        pos.0 += speed.0;
        pos.1 += speed.1;

        if speed.0 > 0 {
            speed.0 -= 1;
        }
        speed.1 -= 1;
    }
    false
}

fn find_minimum_x_speed(start_x: u64) -> u64 {
    let mut remainder = start_x as i64;
    let mut min_initial_x_speed = 0;
    while remainder > 0 {
        min_initial_x_speed+= 1;
        remainder -= min_initial_x_speed;
    }
    min_initial_x_speed as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    fn test_part_1(input: &str, expected: Part1Output) {
        assert_eq!(part_1(parse(input)), expected);
    }

    fn test_part_2(input: &str, expected: Part2Output) {
        assert_eq!(part_2(parse(input)), expected);
    }

    const EXAMPLE: &str = r#"
    target area: x=29..73, y=-248..-194
    "#;

    #[test]
    fn part_1_on_example() {
        test_part_1(
            EXAMPLE,
            30628
        );
    }


    #[test]
    fn will_go_should_work() {
        assert!(will_go((6,9), &((20, 30), (-10, -5))))
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            EXAMPLE,
            4433
        );
    }

}
