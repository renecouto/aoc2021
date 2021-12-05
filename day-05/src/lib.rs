use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coordinate {
    pub x: usize,
    pub y: usize
}

fn number_of_dangerous_places(vents: Vec<(Coordinate, Coordinate)>, count_diagonal: bool) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut duplicated: HashSet<(usize, usize)> = HashSet::new();
    let mut two_tier_insert = |v: (usize, usize)| {
        let is_new = seen.insert(v);
        if !is_new {
            duplicated.insert(v);
        }
    };
    for (start, end) in vents {
        if start.x == end.x {
            let start_y = min(start.y, end.y);
            let end_y = max(start.y, end.y);
            for y in start_y..=end_y {
                two_tier_insert((start.x, y));
            }
        } else if start.y == end.y {
            let start_x = min(start.x, end.x);
            let end_x = max(start.x, end.x);
            for x in start_x..=end_x {
                two_tier_insert((x, start.y));
            }
        }

        else if count_diagonal {
            let y_diff: isize = if start.y < end.y { 1 } else { -1};
            let x_diff: isize = if start.x < end.x { 1 } else { -1};
            for d in 0..=(start.y as isize - end.y as isize).abs() {
                two_tier_insert(((start.x  as isize + x_diff*d) as usize, (start.y as isize + y_diff*d) as usize));
            }
        }
    }
    duplicated.len()
}
fn parse(text: &str) -> Vec<(Coordinate, Coordinate)> {
    let mut res = vec![];
    for line in text.trim().lines() {
        let pair: Vec<&str> = line.trim().split("->").collect();
        let funn = |c: &str| {
            let xy: Vec<&str> = c.split(",").collect();
            if xy.len() != 2 {
                panic!("coordinates must have 2 members!, got {:?}", xy);
            }
            Coordinate{x: xy[0].trim().parse().unwrap(), y: xy[1].trim().parse().unwrap()}
        };
        res.push((funn(pair[0]), funn(pair[1])));
    }
    res
}

fn part_1(text: &str) -> usize {
    number_of_dangerous_places(parse(text), false)
}

fn part_2(text: &str) -> usize {
    number_of_dangerous_places(parse(text), true)
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::*;

    #[test]
    fn parsing_works() {
        let input = r#"
        644,38 -> 644,265
        941,468 -> 941,89
        "#;
        let expected = vec![
            (Coordinate{x: 644, y:  38}, Coordinate{x: 644, y: 265}),
            (Coordinate{x: 941, y: 468}, Coordinate{x: 941, y:  89}),
        ];
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn number_of_dangerous_places_works() {
        let input = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "#;
        let expected = 5;
        let result = part_1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn number_of_dangerous_places_works_on_input() {
        let input = read_to_string("input.txt").unwrap();
        let expected = 5632;
        let result = part_1(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn number_of_dangerous_places_works_using_diagonal() {
        let input = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "#;
        let expected = 12;
        let result = part_2(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn number_of_dangerous_places_works_using_diagonal_on_input() {
        let input = read_to_string("input.txt").unwrap();
        let expected = 22213;
        let result = part_2(&input);
        assert_eq!(result, expected);
    }

}
