// The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers in the diagnostic report.
// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used.

// part2 https://adventofcode.com/2021/day/3#part2

fn sum_by_index(diagnostics: &Vec<&str>) -> Vec<u64> {
    let amount = diagnostics[0].len();
    let mut total_by_index: Vec<u64> = (0..amount).map(|_| 0).collect();
    for numbers in diagnostics.iter().map(|c|to_numbers_unsafe(c)) {
        for (i, n) in numbers.iter().enumerate() {
            total_by_index[i] += *n as u64;
        }
    }
    total_by_index
}

fn sum_by_index_n(diagnostics: &Vec<Vec<u32>>) -> Vec<u64> {
    let amount = diagnostics[0].len();
    let mut total_by_index: Vec<u64> = (0..amount).map(|_| 0).collect();
    for numbers in diagnostics.iter() {
        for (i, n) in numbers.iter().enumerate() {
            total_by_index[i] += *n as u64;
        }
    }
    total_by_index
}

fn bit_vec_to_number(gamma: &Vec<u64>) -> u64 {
    // gamma.reverse();
    let mut gamma_res = 0;
    let len = gamma.len() -1;
    for (i, n) in gamma.iter().enumerate() {
        gamma_res += (2 as u64).pow((len - i) as u32) * n
    }
    gamma_res
}

fn to_numbers_unsafe(d: &str) -> Vec<u32> {
    let radix: u32 = 10;
    d.chars().map(|c| c.to_digit(radix).unwrap()).collect()
}

fn life_support_rating(diagnostics: Vec<&str>) -> u64 {
    let total_by_index = sum_by_index(&diagnostics);

    let mut oxygen_filtered: Vec<Vec<u32>> = diagnostics.iter().map(|c| to_numbers_unsafe(c)).collect();
    let mut co_filtered = oxygen_filtered.clone();

    for i in 0..total_by_index.len() {
        let len = oxygen_filtered.len();
        let most_common_rounded_up: Vec<u32> = sum_by_index_n(&oxygen_filtered).iter().map(|c| {
            if c*2 >= (len as u64) {
                1
            } else {
                0
            }
        }).collect();
        // println!("most common: {:?}", most_common_rounded_up);
        let mut remaining: Vec<Vec<u32>> = vec![];
        for f in oxygen_filtered.iter() {
            if f[i] == most_common_rounded_up[i] {
                remaining.push(f.clone());
            }
        }
        if remaining.is_empty() {
            // println!("ended because of empty");
            break;
        }
        if remaining.len() == 1 {
            oxygen_filtered = remaining.clone();
            // println!("ended because of 1 element");
            break;
        }
        oxygen_filtered = remaining.clone();
        // println!("next remaining: {:?}", &oxygen_filtered);
    }

    for i in 0..total_by_index.len() {
        let len = co_filtered.len();
        let least_common_rounded_down: Vec<u32> = sum_by_index_n(&co_filtered).iter().map(|c| {
            if c*2 >= (len as u64) {
                0
            } else {
                1
            }
        }).collect();
        // println!("most common rounded down: {:?}", least_common_rounded_down);
        let mut remaining: Vec<Vec<u32>> = vec![];
        for f in co_filtered.iter() {
            if f[i] == least_common_rounded_down[i] {
                remaining.push(f.clone());
            }

        }
        if remaining.is_empty() {
            // println!("ended because of empty");
            break;
        }
        if remaining.len() == 1 {
            // println!("ended because of 1 element");
            co_filtered = remaining.clone();
            break;
        }
        // if done { break; }
        co_filtered = remaining.clone();
        // println!("next remaining: {:?}", &oxygen_filtered);
    }
    let ox_res = bit_vec_to_number(&oxygen_filtered[0].iter().map(|c| *c as u64).collect());

    let co_res = bit_vec_to_number(&co_filtered[0].iter().map(|c| *c as u64).collect());
    // println!("ox is {}, co is {}", ox_res, co_res);
    ox_res * co_res
    // let oxygen_filtered_res = oxygen_filtered[0];
}

fn power_consumption(diagnostics: Vec<&str>) -> u64 {
    let len = diagnostics.len();
    let total_by_index: Vec<u64> = sum_by_index(&diagnostics);
    let amount = total_by_index.len();
    let mut gamma: Vec<u64> = (0..amount).map(|_| 0).collect();
    let mut epsilon: Vec<u64> = (0..amount).map(|_| 0).collect();

    for (i, t) in total_by_index.iter().enumerate() {
        if t*2 > (len as u64) {
            gamma[i] = 1;
        } else {
            epsilon[i] = 1;
        }
    }
    bit_vec_to_number(&gamma) * (bit_vec_to_number(&epsilon))
}
#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::*;
    #[test]
    fn part_1_works_on_example() {
        // 00100
        // 11110
        // 10110
        // 10111
        // 10101
        // 01111
        // 00111
        // 11100
        // 10000
        // 11001
        // 00010
        // 01010
        // So, the gamma rate is the binary number 10110, or 22 in decimal.
        // So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
        let sample = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
        let result = power_consumption(sample);
        assert_eq!(result, 198);
    }

    #[test]
    fn part_1_works_on_input() {
        // 00100
        // 11110
        // 10110
        // 10111
        // 10101
        // 01111
        // 00111
        // 11100
        // 10000
        // 11001
        // 00010
        // 01010
        // So, the gamma rate is the binary number 10110, or 22 in decimal.
        // So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
        let result = power_consumption(read_to_string("input.txt").unwrap().split("\n").collect());
        assert_eq!(result, 3320834);
    }

    #[test]
    fn part_2_works_on_example() {
        // 00100
        // 11110
        // 10110
        // 10111
        // 10101
        // 01111
        // 00111
        // 11100
        // 10000
        // 11001
        // 00010
        // 01010
        // So, the gamma rate is the binary number 10110, or 22 in decimal.
        // So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
        let sample = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
        let result = life_support_rating(sample);
        assert_eq!(result, 230);
    }

    #[test]
    fn part_2_works_on_input() {
        let result = life_support_rating(read_to_string("input.txt").unwrap().lines().collect());
        assert_eq!(result, 4481199);
    }
}
