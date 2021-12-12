#![allow(warnings)]
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use crate::Node::{BigCave, SmallCave, Start};

type ProblemInput<'a> = Vec<(Node, Node)>;
fn parse(input: &str) -> ProblemInput {
    input.trim().lines().map(|l| {
        let vs: Vec<&str> = l.trim().split("-").collect();
        (vs[0].trim().clone().parse().unwrap(), vs[1].trim().clone().parse().unwrap())
    }).collect()
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum Node {
    Start,
    End,
    SmallCave(String),
    BigCave(String),
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            s if s.chars().all(|x| x.is_lowercase()) => Ok(SmallCave(s.into())),
            s if s.chars().all(|x| x.is_uppercase()) => Ok(BigCave(s.into())),
            s => Err(format!("did not match on string {}", s)),
        }
    }
}

fn build_graph(p: ProblemInput) -> HashMap<Node, HashSet<Node>> {
    let mut res = HashMap::new();
    p.into_iter().for_each(|(from, to)| {
        if from != Node::End && to != Node::Start {
            res
                .entry(from.clone())
                .or_insert(HashSet::new())
                .insert(to.clone());
        }
        if to != Node::End && from != Node::Start {
            res
                .entry(to.clone())
                .or_insert(HashSet::new())
                .insert(from.clone());
        }
    });
    res
}
type Part1Output = usize;
fn part_1(input: ProblemInput) -> Part1Output {
    let graph = build_graph(input);
    println!("graph is: {:?}", &graph);
    let mut seen: HashSet<Vec<Node>> = HashSet::new();
    let mut completed: HashSet<Vec<Node>> = HashSet::new();
    let mut nexts: Vec<(Vec<Node>, HashSet<Node>)> = vec![(vec![Node::Start], HashSet::new())];
    while let Some((cur_node, has_small)) = nexts.pop() {
        let possible_nexts = graph.get(&cur_node.last().unwrap()).unwrap();
        for n in possible_nexts.iter() {
            let mut new_path = cur_node.clone();
            new_path.push(n.clone());
            if seen.contains(&new_path) || completed.contains(&new_path) {
                continue
            }
            seen.insert(new_path.clone());
            match n {
                Node::End => {completed.insert(new_path); },
                x @ SmallCave(c) if !has_small.contains(x) => {
                    let mut hazmal = has_small.clone();
                    hazmal.insert(x.clone());
                    nexts.push((new_path, hazmal));
                },
                x @ SmallCave(c) if has_small.contains(x) => {
                },
                BigCave(c) => { nexts.push((new_path, has_small.clone())); },
                _ => {
                },
            }

        }
    }
    completed.len()

}

type Part2Output = usize;
fn part_2(input: ProblemInput) -> Part2Output {
    let graph = build_graph(input);
    println!("graph is: {:?}", &graph);
    let mut seen: HashSet<Vec<Node>> = HashSet::new();
    let mut completed: HashSet<Vec<Node>> = HashSet::new();
    let mut nexts: Vec<(Vec<Node>, HashSet<Node>, Option<Node>)> = vec![(vec![Node::Start], HashSet::new(), None)];
    while let Some((cur_node, has_small, duplicated)) = nexts.pop() {
        if seen.contains(&cur_node) {
            println!("seen: {:?}, node: {:?}", &seen, &cur_node);
            continue
        }
        seen.insert(cur_node.clone());


        let possible_nexts = graph.get(&cur_node.last().unwrap()).unwrap();
        for n in possible_nexts.iter() {
            let mut new_path = cur_node.clone();
            new_path.push(n.clone());
            if seen.contains(&new_path) || completed.contains(&new_path) {
                continue
            }
            // seen.insert(new_path.clone());
            match n {
                Node::End => {completed.insert(new_path); },
                x @ SmallCave(c) if !has_small.contains(x) => {
                    let mut hazmal = has_small.clone();
                    hazmal.insert(x.clone());
                    nexts.push((new_path, hazmal, duplicated.clone()));
                },
                x @ SmallCave(c) if has_small.contains(x) && duplicated.is_none() => {
                    nexts.push((new_path, has_small.clone(), Some(x.clone())))
                },
                BigCave(c) => { nexts.push((new_path, has_small.clone(), duplicated.clone())); },
                _ => {},
            }
        }

    }
    // println!("completed: {:?}", &completed);
    completed.len()
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
    const EXAMPLE_1: &str = r#"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
    "#;

    #[test]
    fn part_1_on_example() {
        test_part_1(
            EXAMPLE_1,
            10
        );
    }

    #[test]
    fn part_1_on_input() {
        test_part_1(
            &read_to_string("input.txt").unwrap(),
            4549
        );
    }

    #[test]
    fn part_2_on_example() {
        test_part_2(
            EXAMPLE_1,
            36
        );
    }
    // FIXME this takes 20 seconds on not release, 3 seconds on release. we might be cloning too much, it might make sense trying out linked lists
    #[test]
    fn part_2_on_input() {
        test_part_2(
            &read_to_string("input.txt").unwrap(),
            120535
        );
    }
}
