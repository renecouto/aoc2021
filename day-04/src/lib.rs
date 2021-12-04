use std::collections::{HashMap, HashSet};
use std::str::FromStr;


#[derive(Debug, Clone, Eq, PartialEq)]
struct BingoBoard {
    //FIXME lazy pub to check that logic works
    pub remaining: HashMap<u64, (u64, u64)>,
    pub columns: HashMap<u64,u64>,
    pub rows: HashMap<u64,u64>,
    // pub winner_points: u64,
}

impl BingoBoard {
    fn put_value(&mut self, value: u64) -> Option<u64> {
        match self.remaining.remove(&value) {
            Some((row_idx, col_idx)) => {
                let v = self.columns.get(&col_idx);
                let new_v = v.map_or(1, |x| x + 1);
                self.columns.insert(col_idx, new_v);
                if new_v == 5 {
                    return Some(self.points(value))
                }

                let v = self.rows.get(&row_idx);
                let new_v = v.map_or(1, |x| x + 1);
                self.rows.insert(row_idx, new_v);
                if new_v == 5 {
                    return Some(self.points(value))
                }
            },
            _ => {},
        };
        None
    }

    fn points(&self, current_value: u64) -> u64 {
        self.remaining.keys().sum::<u64>() * current_value
    }
}
impl FromStr for BingoBoard {

    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = BingoBoard{remaining: HashMap::new(), columns: HashMap::new(), rows: HashMap::new()};
        for (row_index, line) in s.trim().lines().filter(|c| !c.is_empty()).enumerate() {
            for (col_index, value) in line.trim().split_whitespace().enumerate() {
                let parsed_value: u64 = match value.parse() {
                    Ok(r) => r,
                    Err(s) => return Err(s.to_string()),
                };
                board.remaining.insert(parsed_value, (row_index as u64, col_index as u64));
            }
        }
        Ok(board)
    }
}


fn from_input(input: &str) -> (Vec<u64>, Vec<BingoBoard>) {
    let (first, next) = input.trim().split_once("\n").unwrap();
    let drawn_numbers = first.split(",").map(|c|c.trim().parse().unwrap()).collect();
    let mut boards = vec![];
    for chunk in next.split("\n\n") {
        boards.push(chunk.parse().unwrap());
    }
    (drawn_numbers, boards)
}
// FIXME error instead of returning 0 on no possible solution
fn play_(numbers: Vec<u64>, boards: &mut Vec<BingoBoard>) -> u64 {

    for n in numbers {
        for board_idx in 0..boards.len() {
            if let Some(res) = boards[board_idx].put_value(n) {
                return res
            }
        }
    }
    0
}

fn play_last(numbers: Vec<u64>, boards: &mut Vec<BingoBoard>) -> u64 {
    let mut remaining: HashSet<usize> = (0..boards.len()).collect();
    for n in numbers {
        for board_idx in remaining.clone().into_iter() {
            if let Some(res) = boards[board_idx].put_value(n) {
                if remaining.len() == 1 {
                    return res
                }
                remaining.remove(&board_idx);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::read_to_string;
    use crate::{BingoBoard, from_input, play_, play_last};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn parsing_a_board_works() {
        let input = r#"
        1   2  3  4  5
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25
        "#;
        let board: BingoBoard = input.parse().unwrap();
        let expected_remaining = [
            (1, (0,0)), (2, (0, 1)), (3, (0, 2)), (4, (0, 3)), (5, (0, 4)),
            (6, (1,0)), (7, (1, 1)), (8, (1, 2)), (9, (1, 3)), (10, (1, 4)),
            (11, (2,0)), (12, (2, 1)), (13, (2, 2)), (14, (2, 3)), (15, (2, 4)),
            (16, (3,0)), (17, (3, 1)), (18, (3, 2)), (19, (3, 3)), (20, (3, 4)),
            (21, (4,0)), (22, (4, 1)), (23, (4, 2)), (24, (4, 3)), (25, (4, 4)),
        ];
        assert_eq!(board, BingoBoard{remaining: HashMap::from(expected_remaining), columns: HashMap::new(), rows: HashMap::new()});
    }

    #[test]
    fn playing_a_board_works() {
        let input = r#"
        1   2  3  4  5
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25
        "#;
        let mut board: BingoBoard = input.parse().unwrap();
        for v in [1,2,3,4] {
            assert_eq!(board.put_value(v), None);
        }
        let sum_of_remaining: u64 = (6..26).sum();
        assert_eq!(board.put_value(5), Some(5*sum_of_remaining));
    }



    #[test]
    fn from_input_works() {
        let input = r#"
        1, 2, 3, 4

        1   2  3  4  5
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25

        1   2  3  4  5
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25
        "#;
        let expected_remaining = [
            (1, (0,0)), (2, (0, 1)), (3, (0, 2)), (4, (0, 3)), (5, (0, 4)),
            (6, (1,0)), (7, (1, 1)), (8, (1, 2)), (9, (1, 3)), (10, (1, 4)),
            (11, (2,0)), (12, (2, 1)), (13, (2, 2)), (14, (2, 3)), (15, (2, 4)),
            (16, (3,0)), (17, (3, 1)), (18, (3, 2)), (19, (3, 3)), (20, (3, 4)),
            (21, (4,0)), (22, (4, 1)), (23, (4, 2)), (24, (4, 3)), (25, (4, 4)),
        ];
        let expected_board = BingoBoard{remaining: HashMap::from(expected_remaining), columns: HashMap::new(), rows: HashMap::new()};
        let (got_drawn_numbers, got_board) = from_input(input);
        assert_eq!(got_drawn_numbers, vec![1,2,3,4]);
        assert_eq!(got_board, vec![expected_board.clone(), expected_board]);
    }

    #[test]
    fn playing_a_game_works() {
        let input = r#"
        1,2,3,4,5

        1   2  3  4  5
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25
        "#;
        let r = from_input(input);
        let mut gb = r.1;
        let r = play_(r.0, &mut gb);
        let sum_of_remaining: u64 = (6..26).sum();
        assert_eq!(r, 5*sum_of_remaining);
    }
    #[test]
    fn playing_a_game_from_input_works() {
        let input = read_to_string("input.txt").unwrap();
        let r = from_input(&input);
        let mut gb = r.1;
        let r = play_(r.0, &mut gb);
        assert_eq!(r, 2496);
    }

    #[test]
    fn playing_a_game_last_works() {
        let input = r#"
        1,2,3,4,5,51,52,53,54,55

        1   2  3  4  5
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25

        51 52 53 54 55
        6   7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25
        "#;
        let r = from_input(input);
        let mut gb = r.1;
        let r = play_last(r.0, &mut gb);
        let sum_of_remaining: u64 = (6..26).sum();
        assert_eq!(r, 55*sum_of_remaining);
    }
    #[test]
    fn playing_a_game_from_input_last_works() {
        let input = read_to_string("input.txt").unwrap();
        let r = from_input(&input);
        let mut gb = r.1;
        let r = play_last(r.0, &mut gb);
        assert_eq!(r, 25925);
    }
}
