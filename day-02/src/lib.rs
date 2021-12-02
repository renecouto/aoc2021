use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up(u64),
    Forward(u64),
    Down(u64),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.split_whitespace().collect::<Vec<&str>>();

        match splitted[..] {
            [x, n] => {
                let direction = match x {
                    "up" => Direction::Up,
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                _ => return Err("invalid direction".into()),
                };
                Ok(direction(n.parse::<u64>().unwrap()))
            },
            _ => Err("no match!".into()),
        }
    }
}

fn final_position_product(items: impl Iterator<Item=Direction>) -> u64 {
    use crate::Direction::*;
    let mut cur_horizontal: u64 = 0;
    let mut cur_vertical: u64 = 0;
    for item in items {
        match item {
            Up(v) => cur_vertical -= v,
            Down(v) => cur_vertical += v,
            Forward(v) => cur_horizontal += v,
        };
    }
    cur_vertical*cur_horizontal
}

fn final_position_product_f(acc: (u64, u64), item: &Direction) -> (u64, u64) {
    use crate::Direction::*;
    match item {
        Up(v) => (acc.0, acc.1 - v),
        Down(v) => (acc.0, acc.1 + v),
        Forward(v) => (acc.0 + v, acc.1),
    }
}

fn final_position_product_2(items: impl Iterator<Item=Direction>) -> u64 {
    use crate::Direction::*;
    let mut cur_horizontal: u64 = 0;
    let mut cur_vertical: u64 = 0;
    let mut aim = 0;
    for item in items {
        match item {
            Up(v) => aim -= v,
            Down(v) => aim += v,
            Forward(v) => {
                cur_horizontal += v;
                cur_vertical += aim*v;
            },
        };
    }
    cur_vertical*cur_horizontal
}


trait FoldExt {
    fn final_position_product(self) -> u64;
}

impl <I> FoldExt for I
where I: Iterator<Item=Direction>
{
    fn final_position_product(self) -> u64 {
        let (a, b) = self.fold((0, 0), |a,b| final_position_product_f(a, &b));
        a*b
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::Direction::*;
    use crate::*;

    #[test]
    fn parsing_works() {
        let input = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",];
        let expected = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let result: Vec<Direction> = input.iter().map(|c|c.parse().unwrap()).collect();
        assert_eq!(result, expected);
    }
    #[test]
    fn it_works_on_sample() {

        // forward 5
        // down 5
        // forward 8
        // up 3
        // down 8
        // forward 2
        // After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
        let items = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let result = final_position_product(items.into_iter());
        assert_eq!(result, 150);
    }

    #[test]
    fn it_works_on_sample_using_fold() {

        // forward 5
        // down 5
        // forward 8
        // up 3
        // down 8
        // forward 2
        // After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
        let items = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let last_position = items.iter().fold((0, 0), |a,b| final_position_product_f(a, b));
        let result = last_position.0*last_position.1;
        assert_eq!(result, 150);
    }

    #[test]
    fn it_works_on_sample_extension() {

        // forward 5
        // down 5
        // forward 8
        // up 3
        // down 8
        // forward 2
        // After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
        let items = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let result = items.into_iter().final_position_product();
        assert_eq!(result, 150);
    }

    #[test]
    fn it_works_on_input() {
        let input = read_to_string("input.txt");
        let items = input.unwrap();
        let items = items.lines().map(|c| c.parse().unwrap());
        let result = final_position_product(items);
        assert_eq!(result, 1815044);
    }

    #[test]
    fn it_works_on_sample_2() {

        // forward 5
        // down 5
        // forward 8
        // up 3
        // down 8
        // forward 2
        // After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
        let items = vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)];
        let result = final_position_product_2(items.into_iter());
        assert_eq!(result, 900);
    }

    #[test]
    fn it_works_on_input_2() {
        let input = read_to_string("input.txt");
        let result = final_position_product_2(input.unwrap().lines().map(|c|c.parse().unwrap()));
        assert_eq!(result, 1739283308);
    }
}
