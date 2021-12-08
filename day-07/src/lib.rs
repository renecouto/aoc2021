use std::cmp::{max, min, Ordering};
use std::num::ParseIntError;

fn parse(s: &str) -> Result<Vec<u64>, ParseIntError> {
    s.trim().split(",").map(|d| d.parse::<u64>()).collect()
}

fn constant_cost(a: u64, b: u64) -> u64 {
    max(a, b) - min(a,b)
}

// The sum of numbers from 1 to n. This can be optimized to do less multiplications if we save
// the previous cost for each crab
fn linear_cost(a: u64, b: u64) -> u64 {
    let dist = max(a, b) - min(a,b);
    dist*(dist+1)/2
}

// this is a brute force approach. There should be a more optimized solution
// for the first part, the position that minimizes cost is the median of the array, but that might be a coincidence,
// since a median can be fractional (in which case both roundings would yield the same result)
// An option would be to build an equation for the total cost as the sum of costs for each crab, and solve that equation
fn crab_fuel_cost(mut crab_positions: Vec<u64>, cost_function: fn(u64, u64) -> u64) -> u64 {
    crab_positions.sort(); // we dont need to sort to get the min and max lol
    let mut c = 0;
    let mut min = None;
    for i in *crab_positions.first().unwrap()..=*crab_positions.last().unwrap() {
        let mut cur = 0;
        for crab in crab_positions.iter() {
            c+=1;
            cur += cost_function(i, *crab);
        }
        if let Some(v)  = min {
            min = Some(cur.min(v));
        } else {
            min = Some(cur);
        }
    }
    dbg!(c);
    min.unwrap() // function should require that crab_positions is not empty, or error in that condition
}

fn sum_of_costs(positions: &[u64], candidate: u64) -> u64 {
    positions.iter().map(|x| linear_cost(*x, candidate)).sum()
}


// we split the problem to look for local minimums in each slice between crabs positions
fn crab_fuel_cost_smart_part_2(mut crab_positions: Vec<u64>) -> u64 {
    let mut crab_positions_full = crab_positions.clone();
    crab_positions.sort();
    crab_positions.dedup(); // we might not need to dedup beforehand, but it makes the code easier to read and reason
    let mut min = None;
    let mut c = 0;
    use std::collections::HashMap;
    let mut costs_by_idx: HashMap<u64, u64> = HashMap::new();
    let mut get_or_insert = |v: u64| {
        if let Some(x) = costs_by_idx.get(&v) {
            *x
        } else {
            let computed = sum_of_costs(&crab_positions_full, v);
            costs_by_idx.insert(v, computed);
            computed
        }
    };
    for i in 0..(crab_positions.len() -1) {
        let mut start = crab_positions[i];
        let mut end = crab_positions[i+1];
        loop {
            c += 1;
            let cost_start = get_or_insert(start);
            let cost_start_plus_one = get_or_insert(start + 1);
            let cost_end = get_or_insert(end);
            let cost_end_minus_one = get_or_insert(end - 1);
            if cost_start <= cost_start_plus_one {
                // parabole is lowering towards the start, so the minimum of this slice should be cost_start
                min = Some(cost_end.min(min.unwrap_or(cost_end)));
                break;
            } else if cost_end <= cost_end_minus_one {
                // the parabole is lowering towards the end, so the minimum of this slice should be cost_end
                min = min.map(|x| x.min(cost_end)).or(Some(cost_end));
                break;
            } else {
                // the parabole has a root inside this slice. we need to try to find a pivot point.
                // we can take the middle of the slice and check its derivate
                let middle = start + (end - start)/2;
                let middle_plus_one = middle + 1;
                let cost_middle = get_or_insert(middle);
                let cost_middle_plus_one = get_or_insert(middle_plus_one);
                match cost_middle.cmp(&cost_middle_plus_one) {
                    Ordering::Less => {end = middle;}
                    Ordering::Equal => {
                        // it is for sure the local minimum
                        min = min.map(|x| x.min(cost_middle)).or(Some(cost_middle));
                        break;
                    }
                    Ordering::Greater => {start = middle;}
                }
            }
        }
    }
    println!("{}",c);

    min.unwrap() // function should require that crab_positions is not empty, or error in that condition
}


// each crab cost is a parable, so the sum of parables is a bigger parable. we then have only one local minimum
fn crab_fuel_cost_simple_part_2(mut crab_positions: Vec<u64>) -> u64 {
    let mut crab_positions_full = crab_positions.clone();
    crab_positions.sort();
    crab_positions.dedup(); // we might not need to dedup beforehand, but it makes the code easier to read and reason
    let mut min = None;
    let mut c = 0;
    use std::collections::HashMap;
    let mut costs_by_idx: HashMap<u64, u64> = HashMap::new();
    let mut get_or_insert = |v: u64| {
        if let Some(x) = costs_by_idx.get(&v) {
            *x
        } else {
            let computed = sum_of_costs(&crab_positions_full, v);
            costs_by_idx.insert(v, computed);
            computed
        }
    };
    let mut start = *crab_positions.first().unwrap();
    let mut end = *crab_positions.last().unwrap();

    loop {

        c += 1;
        let cost_start = get_or_insert(start);
        let cost_start_plus_one = get_or_insert(start + 1);
        let cost_end = get_or_insert(end);
        let cost_end_minus_one = get_or_insert(end - 1);
        if cost_start <= cost_start_plus_one {
            // parable is lowering towards the start, so the minimum of this slice should be cost_start
            min = Some(cost_end.min(min.unwrap_or(cost_end)));
            break;
        } else if cost_end <= cost_end_minus_one {
            // the parabole is lowering towards the end, so the minimum of this slice should be cost_end
            min = min.map(|x| x.min(cost_end)).or(Some(cost_end));
            break;
        } else {
            // the parabole has a root inside this slice. we need to try to find a pivot point.
            // we can take the middle of the slice and check its derivative
            let middle = start + (end - start)/2;
            let middle_plus_one = middle + 1;
            let cost_middle = get_or_insert(middle);
            let cost_middle_plus_one = get_or_insert(middle_plus_one);
            match cost_middle.cmp(&cost_middle_plus_one) {
                Ordering::Less => {end = middle;}
                Ordering::Equal => {
                    // it is for sure the local minimum
                    min = min.map(|x| x.min(cost_middle)).or(Some(cost_middle));
                    break;
                }
                Ordering::Greater => {start = middle;}
            }
        }
    }

    println!("{}",c);

    min.unwrap() // function should require that crab_positions is not empty, or error in that condition
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn test_part_1(input: &str, expected: u64) {
        assert_eq!(crab_fuel_cost(parse(input).unwrap(), constant_cost), expected);
    }

    fn test_part_2(input: &str, expected: u64) {
        assert_eq!(crab_fuel_cost(parse(input).unwrap(), linear_cost), expected);
    }

    fn test_part_2_top(input: &str, expected: u64) {
        assert_eq!(crab_fuel_cost_smart_part_2(parse(input).unwrap()), expected);
    }
    #[test]
    fn it_works() {
        test_part_1(
          "1",
            0
        );
    }

    #[test]
    fn two_crabs() {
        test_part_1(
            "1,2",
            1
        );
    }

    #[test]
    fn on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            345197
        );
    }

    #[test]
    fn two_crabs_p2() {
        test_part_1(
            "1,3",
            2
        );
    }

    #[test]
    fn two_crabs_apart_p2() {
        test_part_2(
            "1,5",
            6
        );
    }

    #[test]
    fn two_crabs_apart_p2_top() {
        test_part_2_top(
            "1,5",
            6
        );
    }

    #[test]
    fn on_input_part_2() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            96361606
        );
    }

    #[test]
    fn on_input_part_2_bench() {
        let input = &read_to_string("input.txt").unwrap();
        for _v in 0..100 { // 9sec 673ms, 10 sec something
            test_part_2(
                input,
                96361606
            );
        }
    }

    #[test]
    fn on_input_part_2_bench_cache_parse() {
        let input = parse(&read_to_string("input.txt").unwrap()).unwrap();
        for _v in 0..100 { // 9sec 673ms, 10 sec something, 9sec 727 with cache parse
            assert_eq!(
                crab_fuel_cost(
                    input.clone(),
                    linear_cost,
                ),
                96361606);
        }
    }

    #[test]
    fn on_input_part_2_top() {
        test_part_2_top(
            &read_to_string("input.txt").unwrap(),
            96361606
        );
    }

    #[test]
    fn on_input_part_2_top_bench() {
        let input = parse(&read_to_string("input.txt").unwrap()).unwrap();
        for _v in 0..100 { // 15sec 126 ms, 7sec 958  - 8sec 79ms with caching. there might be some overhead for dedup/sort, 7sec 854 withou repeat parsing
            assert_eq!(
                crab_fuel_cost_smart_part_2(input.clone()),
                96361606
            );
        }

    }

    #[test]
    fn on_input_part_2_simple_bench() {
        let input = parse(&read_to_string("input.txt").unwrap()).unwrap();
        for _v in 0..100 { // 250ms :clown:
            assert_eq!(
                crab_fuel_cost_simple_part_2(input.clone()),
                96361606
            );
        }

    }

}
