use std::collections::{HashMap, HashSet};

type ProblemInput = Vec<Vec<u64>>;
fn parse(input: &str) -> ProblemInput {
    input.trim().lines().map(|l| l.trim().chars().map(|z|z.to_digit(10).unwrap() as u64).collect()).collect()
}
type Part1Output = u64;
fn gen_next(i: usize, j: usize) -> Vec<(usize,usize)> {
    let i = i as isize;
    let j = j as isize;
    vec![
        (i-1, j-1),
        (i-1, j),
        (i-1, j+1),
        (i, j+1),
        (i+1, j+1),
        (i+1, j),
        (i+1, j-1),
        (i, j-1),
    ].into_iter().filter(|(ni,nj)| {
        let ni = *ni;
        let nj = *nj;
        !(ni < 0 || ni > 9 || nj < 0 || nj > 9)
    }).map(|(i,j)| (i as usize, j as usize)).collect()
}

fn part_1(mut input: ProblemInput) -> Part1Output {
    let x = input.len();
    let y = input[0].len();
    let mut flashes: u64 = 0;
    for _ccc in 0..100 {
        let mut has_summed: Vec<Vec<bool>> = input.clone().into_iter().map(|x| x.into_iter().map(|y| false).collect()).collect();
        let mut is_flashing: Vec<Vec<bool>> = input.clone().into_iter().map(|x| x.into_iter().map(|y| false).collect()).collect();
        run_step(x, y, &mut input, &mut is_flashing, &mut has_summed);
        flashes += is_flashing.iter().map(|x| x.iter().map(|x| if *x {1} else {0}).sum::<u64>()).sum::<u64>();
    }
    input.iter().for_each(|x| println!("{:?}", x));
    flashes
}

fn run_step(x: usize, y: usize, input: &mut Vec<Vec<u64>>, is_flashing: &mut Vec<Vec<bool>>, has_summed: &mut Vec<Vec<bool>>) {
    for i in 0..x {
        for j in 0..y {
            let past = input.get(i).unwrap().get(j).unwrap().clone();
            if !has_summed[i][j] {
                if past == 9 {
                    input[i][j] = 0;
                    is_flashing[i][j] = true;
                    // flash everything nearby :(((
                    let mut nexts: Vec<(usize, usize)> = gen_next(i,j);
                    while let Some((ni, nj)) = nexts.pop() {
                        let past = input.get(ni).unwrap().get(nj).unwrap();
                        if !is_flashing[ni][nj] {
                            let sum_diff = if has_summed[ni][nj] {0} else {1};
                            has_summed[ni][nj] = true;
                            if past + 1 + sum_diff > 9 {
                                is_flashing[ni][nj] = true;
                                nexts.extend(gen_next(ni, nj));
                                input[ni][nj] = 0;
                            } else {
                                input[ni][nj] = past + 1 + sum_diff;
                            }
                        }
                    }
                } else {
                    input[i][j] = past + 1;
                }
                has_summed[i][j] = true;
            }
        }
    }
}

type Part2Output = u64;
fn part_2(mut input: ProblemInput) -> Part2Output {
    let x = input.len();
    let y = input[0].len();
    let mut flashes: u64 = 0;
    for turn in 1.. {
        let mut has_summed: Vec<Vec<bool>> = input.clone().into_iter().map(|x| x.into_iter().map(|y| false).collect()).collect();
        let mut is_flashing: Vec<Vec<bool>> = input.clone().into_iter().map(|x| x.into_iter().map(|y| false).collect()).collect();
        run_step(x, y, &mut input, &mut is_flashing, &mut has_summed);
        let flashed_this_turn = is_flashing.iter().map(|x| x.iter().map(|x| if *x {1} else {0}).sum::<u64>()).sum::<u64>();
        if flashed_this_turn as usize == x * y {
            return turn
        }
    }
    return 0
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
            r#"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
            "#,
            1656
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            1675
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            r#"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526"#,
            195
        );
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            515);
    }
}
