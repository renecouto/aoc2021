use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn calculate<V: Sized + Iterator<Item=u64>, I: IntoIterator<IntoIter=V>>(numbers: I, window_size: usize) -> u64 {
    use crate::*;
    let mut prev: u64 = 0;
    let mut changes: u64 = 0;
    for (i, n) in numbers.into_iter().windowed(window_size).enumerate() {
        let cur: u64 = n.iter().sum();
        if i != 0 && cur > prev {
            changes += 1;
        }
        prev = cur;
    }
    changes
}
use std::collections::VecDeque;
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Windowed<I, V> {
    iter: I,
    window_size: usize,
    elements: VecDeque<V>,
}
impl<I, V> Windowed<I, V> {
    pub fn new(iter: I, window_size: usize) -> Windowed<I, V> {
        Windowed { iter, window_size, elements: VecDeque::new() }
    }
}

trait WindowExt<V> {
    fn windowed(self, window_size: usize) -> Windowed<Self, V> where Self: Sized;
}

impl <I, V> WindowExt<V> for I
    where I: Iterator<Item=V>, I: Sized
{
    fn windowed(self, window_size: usize) -> Windowed<I, V> {
        Windowed::new(self, window_size)
    }
}
impl<I, V> Iterator for Windowed<I, V>
    where
        I: Iterator<Item=V>,
        V: Clone,
{
    type Item = Vec<V>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut is_first = false;
        while self.elements.len() < self.window_size {
            self.elements.push_back(self.iter.next()?);
            is_first = true;
        }
        if !is_first {
            self.elements.pop_front();
            self.elements.push_back(self.iter.next()?)
        }

        Some(self.elements.clone().into())
    }
}

fn main() -> std::io::Result<()> {
    use std::env;
    let q = env::args().into_iter().collect::<Vec<String>>();
    let window_size = q[1].parse().unwrap();
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let res = calculate(reader.lines().map(|l| l.unwrap().parse::<u64>().unwrap()), window_size);
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_windowed_yields_windows() {
        let input: Vec<i32> = vec![1,2,3];
        let expected: Vec<Vec<i32>> = vec![vec![1],vec![2],vec![3]];
        let got: Vec<Vec<i32>> = input.into_iter().windowed(1).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_windowed_yields_windows_of_different_sizes() {
        let input: Vec<i32> = vec![1,2,3];
        let expected: Vec<Vec<i32>> = vec![vec![1,2,3]];
        let got: Vec<Vec<i32>> = input.into_iter().windowed(3).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_windowed_yields_windows_of_different_sizes_multiple_stuff() {
        let input: Vec<i32> = vec![1,2,3];
        let expected: Vec<Vec<i32>> = vec![vec![1,2], vec![2, 3]];
        let got: Vec<Vec<i32>> = input.into_iter().windowed(2).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn calculate_calculates_as_example_1() {
        // 199 (N/A - no previous measurement)
        // 200 (increased)
        // 208 (increased)
        // 210 (increased)
        // 200 (decreased)
        // 207 (increased)
        // 240 (increased)
        // 269 (increased)
        // 260 (decreased)
        // 263 (increased)
        // In this example, there are 7 measurements that are larger than the previous measurement.
        let input: [u64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected: u64 = 7;
        let res = crate::calculate(input, 1);
        assert_eq!(expected, res);
    }

    #[test]
    fn calculate_calculates_as_example_2() {
        // Instead, consider sums of a three-measurement sliding window. Again considering the above example:
        //
        // 199  A
        // 200  A B
        // 208  A B C
        // 210    B C D
        // 200  E   C D
        // 207  E F   D
        // 240  E F G
        // 269    F G H
        // 260      G H
        // 263        H

        // A: 607 (N/A - no previous sum)
        // B: 618 (increased)
        // C: 618 (no change)
        // D: 617 (decreased)
        // E: 647 (increased)
        // F: 716 (increased)
        // G: 769 (increased)
        // H: 792 (increased)
        //
        // In this example, there are 5 sums that are larger than the previous sum.

        let input: [u64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected: u64 = 5;
        let res = crate::calculate(input, 3);
        assert_eq!(expected, res);
    }
}