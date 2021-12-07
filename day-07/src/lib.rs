use std::cmp::{max, min};
use std::num::ParseIntError;

fn parse(s: &str) -> Result<Vec<u64>, ParseIntError> {
    s.trim().split(",").map(|d| d.parse::<u64>()).collect()
}

fn constant_cost(a: u64, b: u64) -> u64 {
    max(a, b) - min(a,b)
}

// The sum of numbers from 1 to n. This can be optimized to do less multiplications if we save
// the previous cost for each crab
fn linear_cost(a: u64, b: u64) -> u64 {
    let dist = max(a, b) - min(a,b);
    dist*(dist+1)/2
}

// this is a brute force approach. There should be a more optimized solution
// for the first part, the position that minimizes cost is the median of the array, but that might be a coincidence,
// since a median can be fractional (in which case both roundings would yield the same result)
// An option would be to build an equation for the total cost as the sum of costs for each crab, and solve that equation
fn crab_fuel_cost(mut crab_positions: Vec<u64>, cost_function: fn(u64, u64) -> u64) -> u64 {
    crab_positions.sort();
    let mut min = None;
    for i in *crab_positions.first().unwrap()..=*crab_positions.last().unwrap() {
        let mut cur = 0;
        for crab in crab_positions.iter() {
            cur += cost_function(i, *crab);
        }
        if let Some(v)  = min {
            min = Some(cur.min(v));
        } else {
            min = Some(cur);
        }
    }
    min.unwrap() // function should require that crab_positions is not empty, or error in that condition
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn test_part_1(input: &str, expected: u64) {
        assert_eq!(crab_fuel_cost(parse(input).unwrap(), constant_cost), expected);
    }

    fn test_part_2(input: &str, expected: u64) {
        assert_eq!(crab_fuel_cost(parse(input).unwrap(), linear_cost), expected);
    }
    #[test]
    fn it_works() {
        test_part_1(
          "1",
            0
        );
    }

    #[test]
    fn two_crabs() {
        test_part_1(
            "1,2",
            1
        );
    }

    #[test]
    fn on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            345197
        );
    }

    #[test]
    fn two_crabs_p2() {
        test_part_1(
            "1,3",
            2
        );
    }

    #[test]
    fn two_crabs_apart_p2() {
        test_part_2(
            "1,5",
            6
        );
    }

    #[test]
    fn on_input_part_2() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            345197
        );
    }

}
