use std::collections::HashSet;

fn parse(raw: &str) -> DisplayData {
    raw.trim().lines().map(|l| {
        let vs: Vec<&str> = l.trim().split("|").collect();
        (vs[0].trim().split(" ").collect(), vs[1].trim().split(" ").collect())
    }).collect()
}
type DisplayData<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

fn part_1(data: DisplayData) -> u64 {
    data.iter().map(|l| l.1.iter().filter(|x|x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7).count() as u64).sum()
}

// FIXME this is overly verbose and uses too many iterations over the input
// We likely don't need to use hashsets and can just search the strings instead, since the size is small
// We should error properly instead of panicking
fn part_2(data: DisplayData) -> u64{
    data.into_iter().map(|l| {
        let combinations = l.0;
        let number_1 = combinations.iter().find(|x| x.len() == 2).unwrap();
        let number_1_chars: HashSet<char> = number_1.chars().collect();
        let number_4 = combinations.iter().find(|x| x.len() == 4).unwrap();
        let number_4_chars: HashSet<char> = number_4.chars().collect();
        let number_7 = combinations.iter().find(|x| x.len() == 3).unwrap();
        let number_7_chars: HashSet<char> = number_7.chars().collect();
        // let _top_char = *number_7_chars.difference(&number_1_chars).last().unwrap();
        let number_8 = combinations.iter().find(|x| x.len() == 7).unwrap();
        let number_8_chars: HashSet<char> = number_8.chars().collect();
        let numbers_2_3_or_5: Vec<(&str, HashSet<char>)> = combinations.clone().into_iter().filter(|x| x.len() == 5).map(|z| (z, z.chars().collect())).collect();
        let number_3 = numbers_2_3_or_5.iter().find(|(st, chars)| chars.difference(&number_1_chars).count() == 3).unwrap();
        let number_2 = numbers_2_3_or_5.iter().find(|(st, chars)| *st != number_3.0 && number_4_chars.difference(chars).count() == 2).unwrap();
        let number_5 = numbers_2_3_or_5.iter().find(|(st, chars)| *st != number_3.0 && number_4_chars.difference(chars).count() == 1).unwrap();

        let numbers_0_6_or_9: Vec<(&str, HashSet<char>)> = combinations.clone().into_iter().filter(|x| x.len() == 6).map(|z| (z, z.chars().collect())).collect();

        let number_9 = numbers_0_6_or_9.iter().find(|(st, chars)| number_2.1.difference(chars).count() == 1 && number_3.1.difference(chars).count() == 0).unwrap();
        let number_6 = numbers_0_6_or_9.iter().find(|(st, chars)| number_1_chars.difference(chars).count() == 1).unwrap();
        let number_0 = numbers_0_6_or_9.iter().find(|(st, chars)| *st != number_9.0 && *st != number_6.0).unwrap();
        use std::collections::HashMap;
        let mut res = HashMap::new();
        let mut sort_hash = |h: HashSet<char>| {
          let mut v: Vec<char> = h.into_iter().collect();
            v.sort();
            v
        };
        res.insert(sort_hash(number_9.1.clone()), 9);
        res.insert(sort_hash(number_8_chars), 8);
        res.insert(sort_hash(number_7_chars), 7);
        res.insert(sort_hash(number_6.1.clone()), 6);
        res.insert(sort_hash(number_5.1.clone()), 5);
        res.insert(sort_hash(number_4_chars), 4);
        res.insert(sort_hash(number_3.1.clone()), 3);
        res.insert(sort_hash(number_2.1.clone()), 2);
        res.insert(sort_hash(number_1_chars), 1);
        res.insert(sort_hash(number_0.1.clone()), 0);
        let values = l.1;
        let mut rez = 0;
        for (i, v) in values.iter().enumerate() {
            let mut pp = v.chars().collect::<Vec<char>>();
            pp.sort();
            let parsed = res.get(&pp).unwrap();
            rez += 10_i32.pow(3-(i as u32)) * parsed;
        }
        // values[0].chars().collect();

        // let number_0 = numbers_0_or_6_or_9.iter().find(|x| x.);
        rez as u64
    }).sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    fn test_part_1(input: &str, expected: u64) {
        assert_eq!(part_1(parse(input)), expected);
    }
    fn test_part_2(input: &str, expected: u64) {
        assert_eq!(part_2(parse(input)), expected);
    }

    #[test]
    fn parsing_works() {
        let input = r#"
        cdafg dage fgdaec cdbfgae cge gcbdfa fdceb gfceab ge ecfgd | eg eg dfecag ge
        "#;
        let expected =
        vec![(vec!["cdafg", "dage", "fgdaec", "cdbfgae", "cge", "gcbdfa", "fdceb", "gfceab", "ge", "ecfgd"], vec!["eg", "eg", "dfecag", "ge"])]
            ;
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn example_1_works() {
        let input = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        test_part_1(
          input,
            26
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(&read_to_string("input.txt").unwrap(), 264);
    }

    #[test]
    fn part_2_on_example() {
        let input = r#"
        acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
            "#;
        test_part_2(&input, 5353);
    }

    #[test]
    fn part_2_on_input() {

        test_part_2(&read_to_string("input.txt").unwrap(), 1063760);
    }
}
