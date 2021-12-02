#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up(u64),
    Forward(u64),
    Down(u64),
}
fn parse<'a>(s: impl Iterator<Item=&'a str>) -> Vec<Direction> {
    s.map(|x| {
        let splitted = x.split_whitespace().collect::<Vec<&str>>();

        match splitted[..] {
            [x, n] => {
                let direction = match x {
                    "up" => Direction::Up,
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    _ => panic!("invalid direction"),
                };
                direction(n.parse::<u64>().unwrap())
            },
            _ => panic!("no match!"),
        }
    }).collect::<Vec<Direction>>()

}

fn final_position_product(items: Vec<Direction>) -> u64 {
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
        let result = parse(input.into_iter());
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
        let result = final_position_product(items);
        assert_eq!(result, 150);
    }

    #[test]
    fn it_works_on_input() {
        let input = read_to_string("input.txt");
        let result = final_position_product(parse(input.unwrap().lines().into_iter()));
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
        let result = final_position_product_2(parse(input.unwrap().lines().into_iter()).into_iter());
        assert_eq!(result, 1739283308);
    }
}
