type ProblemInput = Vec<Vec<char>>;

fn parse(input: &str) -> ProblemInput {
    input.trim().lines().map(|c| c.trim().chars().collect()).collect()
}
type Part1Output = u64;
fn part_1(input: ProblemInput) -> Part1Output {
    let mut total = 0;
    input.into_iter().for_each(|chars| {
        let mut stack = vec![];
        for ch in chars.into_iter() {
            if ch == '[' || ch == '(' || ch == '{' || ch == '<' {
                stack.push(ch);
            } else {
                let mapped = match ch.clone() {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    c => panic!("found illegal char, {}", c.clone()),
                };
                if stack.last() == Some(&mapped) {
                    stack.pop();
                } else {
                    let to_add = match ch {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        c => panic!("found illegal char, {}", c),
                    };
                    total += to_add;
                    break
                }
            }

        }
    });
    total
}

type Part2Output = u64;
fn part_2(input: ProblemInput) -> Part2Output {
    let mut totals: Vec<u64> = input.into_iter().enumerate().map(|(i, chars)| {
        let mut stack = vec![];
        for ch in chars.into_iter() {
            if ch == '[' || ch == '(' || ch == '{' || ch == '<' {
                stack.push(ch);
            } else {
                let mapped = match ch.clone() {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    c => panic!("found illegal char, {}", c.clone()),
                };
                if stack.last() == Some(&mapped) {
                    stack.pop();
                } else {
                    println!("returning none on line {}", i);
                    return None
                }
            }
        }
        
        let mut line_total = 0;
        for v in stack.into_iter().rev() {
            let mapped = match v.clone() {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("bad character, {}", v.clone()),
            };
            line_total = 5*line_total + mapped;
        }
        Some(line_total)
    }).filter(|x| x.is_some() ).map(|x| x.unwrap()).collect();
    totals.sort();
    totals[(totals.len()-1)/2]
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
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
                    "#,
                    26397
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            364389
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            r#"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
                    "#,
                    288957
        );
    }

    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            2870201088
        );
    }
}
