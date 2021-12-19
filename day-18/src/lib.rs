#![allow(warnings)]
use std::collections::{HashMap, HashSet};
use indextree::Arena;
use crate::LRX::{CloseBracket, Content, OpenBracket};

/*
DISCLAIMER

this solution is quite bad lol.
It hit me that doing pointer arithmetics is as unsafe as anything.
Yes, it is the simplest way to take slices that we know should not fail, but they are really hard to understand
Also, using indexes on arrays makes it hard to refactor the code to return Result's instead of panicking
An alternative would be to use the iterator, .get(index) -> Option, taking slices...

Ideally, once we parse the problem, the structure should be typesafe: all allowed operations should succeed


 */
// This can be done using a tree or something instead
#[derive(Debug, Clone, Eq, PartialEq)]
enum LRX {
    // content with value and height
    Content(u8, u8),
    OpenBracket(u8),
    CloseBracket(u8),
}
type ProblemInput = Vec<Vec<(u8, u8)>>;
fn parse(input: &str) -> ProblemInput {
    input.trim().lines().map(|l|{
        let mut height = 0;
        let mut ns = vec![];
        for c in l.trim().chars() {
            match c {
                '[' => height+=1,
                ']' => height -=1,
                ',' => {},
                number => ns.push((number.to_digit(10).unwrap() as u8, height)),
            }
        }
        ns
    }).collect()
}

fn parse2(input: &str) -> Vec<Vec<LRX>> {
    input.trim().lines().map(|l|{
        let mut height = 0;
        let mut ns = vec![];
        for c in l.trim().chars() {
            match c {
                '[' => {ns.push(OpenBracket(height)); height+=1;},
                ']' => {height -=1;ns.push(CloseBracket(height));},
                ',' => {},
                number => ns.push(Content(number.to_digit(10).unwrap() as u8, height)),
            }
        }
        ns
    }).collect()
}

fn final_sum2(input: Vec<Vec<LRX>>) -> Vec<LRX> {
    let mut it = input.into_iter();
    let mut res = it.next().unwrap();
    for mut n in it {
        // println!("{:?}", &res);
        // println!("{:?}", &n);
        res.iter_mut().for_each(|x| match x {
            Content(_, h) => {*h += 1}
            OpenBracket(h) => {*h+=1}
            LRX::CloseBracket(h) => {*h+=1}
        });
        n.iter_mut().for_each(|x| match x {
            Content(_, h) => {*h += 1}
            OpenBracket(h) => {*h+=1}
            LRX::CloseBracket(h) => {*h+=1}
        });
        res.extend(n);
        res.insert(0, OpenBracket(0));
        res.push(CloseBracket(0));
        loop {
            let mut ran = false;
            if explode2(&mut res) {
                ran = true;
            } else if split2(&mut res) {
                ran = true;
            }
            if !ran {
                break
            }
        }
    }
    res
}

type Part1Output = u64;
fn final_sum(input: ProblemInput) -> Vec<(u8, u8)> {
    let mut it = input.into_iter();
    let mut res = it.next().unwrap();
    for n in it {
        res = res.into_iter().map(|(c, h)| (c, h+1)).collect();
        res.extend(n.into_iter().map(|(c, h)| (c, h+1)));
        loop {
            let mut ran = false;
            if should_explode(&res) {
                explode(&mut res);
                ran = true;
            } else if should_split(&res) {
                split(&mut res);
                ran = true;
            }
            if !ran {
                break
            }
        }
        println!("{:?}", &res);
    }
    res
}



fn part_1(input: Vec<Vec<LRX>>) -> Part1Output {
    magnitude(&final_sum2(input))
}

fn should_split(v: &Vec<(u8, u8)>) -> bool {
    v.iter().any(|(x, y)| *x > 9)
}

fn split(v: &mut Vec<(u8, u8)>) {
    let mut i = 0;
    while i < v.len() {
        if v[i].0 > 9 {
            let n: f32 = (v[i].0 as f32) / 2.0;

            v.insert(i+1, (n.ceil() as u8, v[i].1 + 1));
            v[i] = (n.floor() as u8, v[i].1 + 1);
            return
        }
        i += 1;
    }
}

fn split2(v: &mut Vec<LRX>) -> bool {
    let mut i = 0;
    while i < v.len() {
        if let Content(c, h) = v[i].clone() {
            if c > 9 {
                let n: f32 = (c as f32) / 2.0;
                v.insert(i+1, CloseBracket(h));
                v[i]= Content(n.ceil() as u8,  h+1);
                v.insert(i, Content(n.floor() as u8,  h+1));
                v.insert(i, OpenBracket(h));
                return true
            }
        }
        i += 1;
    }
    false

}
fn magnitude_(v: &[LRX], height: u8) -> u64 {
    let (lhs, vz) = if let Content(c,h) = v[1] {
        (3*(c as u64), 1)
    } else if let OpenBracket(h)  = v[1].clone() {
        let mut x = 1;
        loop {
            if let CloseBracket(y) = v[x] {
                if y == h {
                    break
                }
            }
            x+=1;
        }
        (3*magnitude_(&v[1..=x],0), x)
    } else {
        unreachable!("cuu");
    };
    let rhs = if let Content(c2,h2) = v[1+ vz] {
        2*(c2 as u64)
    } else if let OpenBracket(h)  = v[1+vz].clone() {
        let mut x = 1 + vz;
        loop {
            if let CloseBracket(y) = v[x] {
                if y == h {
                    break
                }
            }
            x+=1;
        }
        2*magnitude_(&v[1+vz..=x],0)
    } else {
        unreachable!("cuuu");
    };
    rhs + lhs
}

fn magnitude(v: &[LRX]) -> u64 {
    magnitude_(v, 0)
}
fn should_explode(v: &Vec<(u8, u8)>) -> bool {
    v.iter().any(|(x, y)| *y > 4)
}

fn explode(v: &mut Vec<(u8, u8)>) {
    let mut i = 0;
    while i < v.len() {
        if v[i].1 > 4 {
            if i > 0 {
                v[i-1].0 += v[i].0;
            }
            if i < v.len() - 2 {
                v[i+2].0 += v[i+1].0;
            }
            if i < v.len() - 1 {
                v.remove(i+1);
            }
            v[i] = (0, v[i].1 -1);
        }
        i += 1;
    }
}

fn explode2(v: &mut Vec<LRX>) -> bool {
    let mut i = 0;
    while i < v.len() {
        if let LRX::Content(c, h) = &v[i].clone()  {
            if *h > 4 {
                let mut prev_n = i-1;
                while prev_n > 0 {
                    match v[prev_n] {
                        Content(c, v) => {break}
                        _ => {prev_n -=1}
                    }
                }

                let mut next_n = i+2;
                while next_n < v.len() {
                    match v[next_n] {
                        Content(c, v) => {break}
                        _ => {next_n +=1}
                    }
                }

                if prev_n > 0 {
                    match v[prev_n] {
                        Content(ref mut cz,h) => {*cz += c},
                        _ => unreachable!(),
                    };
                }

                if  next_n < v.len() {
                    let cx = match v[i+1] {
                        Content(c, h) => c,
                        _ => unreachable!(),
                    };
                    match v[next_n] {
                        Content(ref mut cz,h) => {*cz += cx},
                        _ => unreachable!(),
                    };
                }
                v.remove(i+2);
                v.remove(i+1);
                v.remove(i-1);

                v[i-1] = Content(0, h-1);
                return true
            }
        }
        i += 1;
    }
    false
}
type Part2Output = u64;
fn part_2(input: Vec<Vec<LRX>>) -> Part2Output {
    let mut max =  0;
    for i in 0..input.len() {
        for x in 0..input.len() {
            max = max.max(part_1(vec![input[i].clone(), input[x].clone()]));
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    fn test_part_1(input: &str, expected: Part1Output) {
        assert_eq!(part_1(parse2(input)), expected);
    }

    fn test_part_2(input: &str, expected: Part2Output) {
        assert_eq!(part_2(parse2(input)), expected);
    }

    const EXAMPLE: &str = r#"
    [[[[4,3],4],4],[7,[[8,4],9]]]
    [1,1]
    "#;
    const C: &str = "[[1,1], 1]";

    #[test]
    fn explode_works_left() {
        let v = "[[[[[9,8],1],2],3],4]";
        let mut i = parse(v);
        explode(&mut i[0]);
        assert_eq!(i[0], parse("[[[[0,9],2],3],4]")[0]);
    }

    #[test]
    fn explode_works_right() {
        let v = "[[[[[9,8],1],2],3],4]";
        let mut i = parse(v);
        i[0].reverse();
        explode(&mut i[0]);
        let mut expected = &mut parse("[[[[0,9],2],3],4]")[0];
        expected.reverse();
        assert_eq!(&i[0], expected);
    }

    #[test]
    fn explode2_works_right() {
        let v = "[[[[[9,8],1],2],3],4]";
        let mut i = parse2(v);
        // i[0].reverse();
        explode2(&mut i[0]);
        let mut expected = &mut parse2("[[[[0,9],2],3],4]")[0];
        // expected.reverse();
        assert_eq!(&i[0], expected);
    }

    #[test]
    fn split_works() {
        let i = parse(EXAMPLE);
        let i = final_sum(i);
        assert_eq!(i, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0]);
    }


    #[test]
    fn split2_works() {
        let mut i = vec![OpenBracket(0), Content(11, 1),


                                                    Content(2,1),
                                CloseBracket(0)];
        split2(&mut i);
        assert_eq!(i, parse2("[[5,6],2]")[0]);
    }

    #[test]
    fn finalsum2_works() {
        let i = parse2(EXAMPLE);
        let i = final_sum2(i);
        assert_eq!(i, parse2("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0]);
    }

    #[test]
    fn magnitude_works() {

        let i = parse2("[[9,1],[1,9]]")[0].clone();
        assert_eq!(magnitude(&i), 129);
    }

    #[test]
    fn magnitude_works_left() {
        let i = parse2("[[2,1],2]")[0].clone();
        // 3 * (3* 2 + 2* 2) + 2 * 2
        // 3 * ( 6 + 4) + 4
        // 3 * 24 + 4
        // 28
        assert_eq!(magnitude(&i), 28);
    }

    #[test]
    fn split_works_right() {
        // let v = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let mut input = vec![(1,1), (11,1)];
        split(&mut input);
        assert_eq!(&input, &parse("[1,[5,6]]")[0]);
    }

    #[test]
    fn split_works_left() {
        // let v = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let mut input = vec![(11,1), (1,1)];
        split(&mut input);
        assert_eq!(&input, &parse("[[5,6],1]")[0]);
    }
    #[test]
    fn final_sum_works() {
        let v =
            r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#;
        let r = final_sum(parse(v));
        let expected = &parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")[0];
        assert_eq!(&r, expected);
    }

    #[test]
    fn final_sum2_works_big_example() {
        let v =
            r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#;
        let r = final_sum2(parse2(v));
        let expected = &parse2("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")[0];
        assert_eq!(&r, expected);
    }


    #[test]
    fn part_1_on_example() {
        test_part_1(
            EXAMPLE,
            Part1Output::default()
        );
    }


    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            4243
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            EXAMPLE,
            Part2Output::default()
        );
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            4701
        );
    }
}
