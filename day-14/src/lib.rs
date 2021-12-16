use std::collections::{HashMap, HashSet};

type ProblemInput = (Vec<char>, HashMap<(char, char), char>);
fn parse(input: &str) -> ProblemInput {
    let (seq, map) = input.trim().split_once("\n\n").unwrap();
    (
        seq.trim().chars().collect(),
        {
            let mut v = HashMap::new();
            map.trim().lines().map(|l| {
                let (k, v) = l.trim().split_once("->").unwrap();
                let vs: Vec<char> = k.trim().chars().collect();
                ((vs[0], vs[1]), v.trim().chars().next().unwrap())
            }).for_each(|(k, z)| {v.insert(k, z);});
            v
        }
    )
}
type Part1Output = u64;
// Brute force solution, eager calculation without caching results, would need 13 terabytes of memory to calculate the part2 on input.txt
fn polymer_ocurrence_calculation(input: ProblemInput, iterations: u64) -> Part1Output {
    let (mut char_seq, map) = input;
    for _v in 0..iterations {
        let mut new = vec![];
        new.push(char_seq[0]);
        for i in 1..char_seq.len() {
            if let Some(x) = map.get(&(char_seq[i - 1], char_seq[i])) {
                new.push(x.clone());
            }
            new.push(char_seq[i])
        }
        char_seq = new;
    }

    let mut wc: HashMap<char, u64> = HashMap::new();
    for c in char_seq.into_iter() {
        let x = wc.entry(c).or_insert(0);
        *x += 1;
    }
    let (max, min) = wc
        .into_iter()
        .fold(
            (0 as u64, u64::MAX),
            |(max_x, min_x), (k, v)| {
                (max_x.max(v), min_x.min(v))
            });
    max - min
}


fn dynamic_polymer_ocurrence_calculation(input: ProblemInput, iterations: u32) -> Part1Output {
    let (mut char_seq, map) = input;
    let mut calculated: HashMap<(u32, (char, char)), HashMap<char, u64>> = HashMap::new();
    let mut to_calc: Vec<(u32, (char, char))> = vec![];
    // Put all the last steps we want to calculate in a stack for lazy calculation
    for i in 1..char_seq.len() {
        to_calc.push((iterations.clone(), (char_seq[i-1], char_seq[i])));
    }

    while let Some((depth,(start, end))) = to_calc.pop() {
        if depth == 1 {
            // this is the cornerstone on top of which everything is calculated and cached
            // we only put the middle character on the calculated cache so we don't have to subtract the start and end everytime
            let mut trivial_calc = HashMap::from([(map.get(&(start, end)).unwrap().clone(), 1)]);
            calculated.insert((1, (start, end)), trivial_calc);
            continue
        }
        let child = map.get(&(start, end)).unwrap().clone();
        let start_child =  calculated.get(&(depth - 1, (start, child)));
        let child_end =  calculated.get(&(depth - 1, (child, end)));
        if start_child.is_some() && child_end.is_some() {
            let mut wc = start_child.unwrap().clone();
            for (k, v) in child_end.unwrap().into_iter() {
                *wc.entry(k.clone()).or_insert(0) += v.clone();
            }
            *wc.entry(child).or_insert(0) += 1;
            calculated.insert((depth, (start, end)), wc);
        } else {
            // VERY IMPORTANT we push the calculation back on the stack so it we calculate it once we have the needed parts
            to_calc.push((depth, (start, end)));
            if start_child.is_none() {
                to_calc.push((depth - 1, (start, child)));
            }
            if child_end.is_none() {
                to_calc.push((depth - 1, (child, end)));
            }
        }

    }
    let mut res = HashMap::new();
    // merge all ocurrence maps
    for i in 1..char_seq.len() {
        let past = calculated.get(&(iterations.clone(), (char_seq[i-1], char_seq[i]))).unwrap();
        for (k, v) in past.into_iter() {
            *res.entry(k.clone()).or_insert(0) += v;
        }
    }
    // since the calculated cache has only the children, we include the parents in the ocurrence counting
    for c in char_seq {
        *res.entry(c).or_insert(0) += 1;
    }
    let (max, min) = max_min_wc(res);
    max - min
}

fn max_min_wc(x: HashMap<char, u64>) -> (u64, u64) {
    x
        .into_iter()
        .fold(
            (0 as u64, u64::MAX),
            |(max_x, min_x), (k, v)| {
                (max_x.max(v), min_x.min(v))
            })
}

fn part_1(input: ProblemInput) -> Part1Output {
    // polymer_ocurrence_calculation(input, 10)
    dynamic_polymer_ocurrence_calculation(input, 10)
}

type Part2Output = u64;
fn part_2(input: ProblemInput) -> Part2Output {
    dynamic_polymer_ocurrence_calculation(input, 40)
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
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "#;

    #[test]
    fn part_1_on_example() {
        test_part_1(
            EXAMPLE,
            1588
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            3230
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            EXAMPLE,
            2188189693529
        );
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            3542388214529
        );
    }
}
