use std::collections::HashSet;

#[derive(Clone)]
enum Fold {
    Left(u64),
    Up(u64)
}
type ProblemInput = (HashSet<(u64, u64)>, Vec<Fold>);
fn parse(input: &str) -> ProblemInput {
    let (positions, folds) = input.split_once("\n\n").unwrap();
    (
        positions.trim().lines().map(|l|{
            let cs: Vec<&str> = l.trim().split(",").collect();
            (cs[0].trim().parse().unwrap(), cs[1].trim().parse().unwrap())
        }).collect(),
        folds.trim().lines().map(|l| {
            let (along, n) =  l.trim().split_once("=").unwrap();
            let fold_index: u64 = n.trim().parse().unwrap();
            let fold_char = along.trim().chars().last().unwrap();
            match fold_char {
                'x' => Fold::Left(fold_index),
                'y' => Fold::Up(fold_index),
                _ => unreachable!(),
            }
        }).collect()
    )

}
type Part1Output = usize;
fn part_1(input: ProblemInput) -> Part1Output {
    let (mut points, folds) = input;
    let fold = folds.first().unwrap();
    apply_fold(&mut points, fold.clone());

    points.len()
}

fn apply_fold(points: &mut HashSet<(u64, u64)>, fold: Fold) {
    for (x, y) in points.clone().into_iter() {
        match &fold {
            &Fold::Left(idx) if x > idx => {
                points.remove(&(x, y));
                points.insert((idx - (x-idx), y));
            },
            &Fold::Up(idx) if y > idx => {
                points.remove(&(x, y));
                points.insert((x, idx - (y-idx)));
            },
            _ => {continue}
        }
    }
}
type Part2Output = usize;
fn part_2(input: ProblemInput) -> Part2Output {
    let (mut points, folds) = input;
    folds.into_iter().for_each(|f| apply_fold(&mut points, f));
    let pv: Vec<(u64, u64)> = points.clone().into_iter().collect();
    let max_x = pv.iter().map(|x|x.0).fold(0 as u64, |x, acc| x.max(acc));
    let max_y = pv.iter().map(|x|x.1).fold(0 as u64, |x, acc| x.max(acc));
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if points.contains(&(x, y)) {"#"} else {" "});
        }
        println!("");
    }
    points.len()
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
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "#;

    #[test]
    fn part_1_on_example() {
        test_part_1(
            EXAMPLE,
            17
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            712
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            EXAMPLE,
            123
        );
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            123
        );
    }
}
