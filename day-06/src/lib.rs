fn number_of_fishes(fishes: Vec<usize>, n: u64) -> u64 {
    let mut fishes_by_day: [u64;9] = [0,0,0,0,0,0,0,0,0];
    for fish in fishes {
        fishes_by_day[fish] += 1;
    }
    for v in 0..n {
        let mut aux = fishes_by_day.clone();
        let to_add = fishes_by_day[0];
        aux[8] = fishes_by_day[0];
        for i in 1..9 {
            aux[i-1] = fishes_by_day[i];
        }
        aux[6] = aux[6] + to_add;
        fishes_by_day = aux;
    }
    fishes_by_day.iter().sum()
}

fn parse(s: &str) -> Vec<usize> {
    s.trim().split(",").map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    fn eq_test(input: (&str, u64), expected: u64) {
        assert_eq!(number_of_fishes(parse(input.0), input.1), expected);
    }
    #[test]
    fn parse_works() {
        assert_eq!(parse("1,2,3"), vec![1,2,3]);
    }
    #[test]
    fn on_sinlge_fish_1_turn() {
        eq_test(
            ("1", 1),
            1
        );
    }

    #[test]
    fn on_sinlge_fish_2_turns() {
        eq_test(
            ("1", 2),
            2
        );
    }

    #[test]
    fn on_example() {
        eq_test(
            ("3,4,3,1,2", 18),
            26
        );
    }

    #[test]
    fn on_example_80_days() {
        eq_test(
            ("3,4,3,1,2", 80),
            5934
        );
    }


    #[test]
    fn on_input() {
        eq_test(
            (&read_to_string("input.txt").unwrap(), 80),
            360268
        );
    }

    #[test]
    fn on_input_part_2() {
        eq_test(
            (&read_to_string("input.txt").unwrap(), 256),
            1632146183902
        );
    }
}
