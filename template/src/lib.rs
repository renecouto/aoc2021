use std::collections::{HashMap, HashSet};

type ProblemInput = u64;
fn parse(input: &str) -> ProblemInput {
    todo!()
}
type Part1Output = u64;
fn part_1(input: ProblemInput) -> Part1Output {
    todo!()
}

type Part2Output = u64;
fn part_2(input: ProblemInput) -> Part2Output {
    todo!()
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

    #[test]
    fn part_1_on_example() {
        test_part_1(
            r#""#,
            todo!()
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            todo!()
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            r#""#,
            todo!()
        );
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            todo!()
        );
    }
}
