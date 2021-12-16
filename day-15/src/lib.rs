use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;
type ProblemInput = Vec<Vec<u32>>;
fn parse(input: &str) -> ProblemInput {
    input
        .trim()
        .lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}
fn gen_next(cur: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let (x, y) = cur;
    let mut res = vec![];
    if x != 0 {
        res.push((x-1, y));
    }
    if y != 0 {
        res.push((x, y-1));
    }
    if x != max_x {
        res.push((x+1, y));
    }
    if y != max_y {
        res.push((x, y + 1));
    }
    res

}
type Part1Output = u64;
fn part_1(input: ProblemInput) -> Part1Output {
    find_path(input)
}

fn find_path(input: ProblemInput) -> u64 {
    let mut v: BinaryHeap<Reverse<(u64, (usize, usize))>> = BinaryHeap::new();
    let max_x = input.len() - 1;
    let max_y = input[0].len() - 1;
    let start = (0, (0,0));
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    v.push(Reverse(start));


    while let Some(Reverse((cost, (x, y)))) = v.pop() {
        if seen.contains(&(x, y)) {
            continue
        }
        seen.insert((x, y));
        if x == max_x && y == max_y {
            return cost
        }
        for &(cx, cy) in gen_next((x, y), max_x, max_y).iter() {

            v.push(Reverse((cost + input[cx][cy] as u64, (cx, cy))));
        }
    }
    return 0
}

type Part2Output = u64;
fn part_2(mut input: ProblemInput) -> Part2Output {

    let mut complete_1: Vec<Vec<u32>> = input.clone().into_iter()
        .map(|mut l| {
            let mut start = l.clone();
            for _v in 0..4 {
                let new = start.into_iter().map(|x| if x == 9 { 1 } else {x + 1});

                l.extend(new.clone());
                start = new.collect();
            }
            l
        }).collect();
    let mut start = complete_1.clone();
    for _v in 0..4 {
        let new: Vec<Vec<u32>> = start.clone().into_iter().map(|x| x.into_iter().map(|x| if x == 9 { 1 } else {x + 1}).collect()).collect();
        complete_1.extend(new.clone());
        start = new;
    }

    find_path(complete_1)


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
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "#;

    #[test]
    fn part_1_on_example() {
        test_part_1(EXAMPLE, 40);
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            40
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(EXAMPLE, 315);
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            2938,
        );
    }
}
