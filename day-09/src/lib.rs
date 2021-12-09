use std::cell::Cell;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|x| x.trim().chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect())
        .collect()
}

fn count_lowest_points(map: Vec<Vec<u32>>) -> u64 {
    let mut total = 0;
    let max_x = map.len() -1;
    let max_y = map[0].len() -1;
    for i in 0..(max_x+1) {
        for j in 0..(max_y+1) {
            let cur = map[i][j];
            let smaller_than_to_the_right = (i+1> max_x || cur < map[i+1][j]);
            let smaller_than_to_the_left = (i < 1 || cur < map[i-1][j]);
            let smaller_than_above = (j+1 > max_y || cur < map[i][j+1]);
            let smaller_than_below = (j < 1 || cur < map[i][j-1]);
            if  smaller_than_above && smaller_than_below && smaller_than_to_the_left && smaller_than_to_the_right {
                total += cur as u64 + 1;
            }
        }
    }
    total
}

fn p2_build_links(map: Vec<Vec<u32>>) -> HashMap<(usize, usize), (usize, usize)> {
    let max_x = map.len() -1;
    let max_y = map[0].len() -1;
    let mut position_map: HashMap<(usize, usize),(usize, usize)> = HashMap::new();
    for i in 0..(max_x+1) {
        for j in 0..(max_y+1) {
            let cur = map[i][j];
            // we can optimize the solution by trying to find the root here
            if cur == 9 {

            } else if i+1 <  max_x && cur > map[i+1][j] {
                position_map.insert ((i, j),(i+1, j));
            } else if i > 0  && cur > map[i -1][j] {
                position_map.insert ((i, j),(i-1, j));
            } else if j > 0  && cur > map[i][j -1] {
                position_map.insert ((i, j), (i, j-1));
            } else if j < max_y  && cur > map[i][j + 1] {
                position_map.insert ((i, j), (i, j + 1));
            } else {
                position_map.insert((i, j), (i,j));
            }
        }
    }
    position_map
}

// I have no idea why this works and the dont_cache_all doesnt :( also there might be more clones than necessary
fn union_find_sizes_clone_keys(mut position_map: HashMap<(usize, usize), (usize, usize)>) -> HashMap<(usize, usize), u128> {
    let mut c = 0;
    let mut count_by_root: HashMap<(usize, usize), u128> = HashMap::new();
    position_map.clone().keys().for_each(|k|{
        let mut root = position_map.get(k).unwrap().clone();
        loop{
            c+=1;
            let new_root = position_map.get(&root).unwrap().clone();
            if new_root == root {
                break
            }
            root = new_root;
        }
        let old_count =  count_by_root.get(&root);
        let new_count = old_count.unwrap_or(&0) + 1;
        count_by_root.insert(root, new_count);
        position_map.insert(*k, root);
    });
    print!("{}", c); //20514
    count_by_root
}

// FIXME it runs as many iterations as not caching LOL
fn union_find_sizes_dont_cache_all(mut position_map: HashMap<(usize, usize), (usize, usize)>) -> HashMap<(usize, usize), u128> {
    let mut c = 0;
    let mut count_by_root= HashMap::new();
    for (_k, v) in position_map.clone().iter_mut() {
        let mut root = v;
        loop{
            c+=1;
            let new_root = position_map.get(root).unwrap().clone();
            if new_root == *root {
                break
            }
            *root = new_root;
        }
        let old_count =  count_by_root.get(root);
        let new_count = old_count.unwrap_or(&0) + 1;
        count_by_root.insert(*root, new_count);
    }
    println!("{}", c); // 32660
    count_by_root
}

fn union_find_sizes_dont_cache_any(mut position_map: HashMap<(usize, usize), (usize, usize)>) -> HashMap<(usize, usize), u128> {
    let mut count_by_root: HashMap<(usize, usize), u128> = HashMap::new();
    let mut c = 0;
    for (_k, v) in position_map.iter() {
        let mut root = v;
        loop{
            c += 1;
            let new_root = position_map.get(&root).unwrap();
            if new_root == root {
                break
            }
            root = new_root;
        }
        let old_count =  count_by_root.get(&root);
        let new_count = old_count.unwrap_or(&0) + 1;
        count_by_root.insert(*root, new_count);
    }
    print!("{}", c); //32660
    count_by_root
}

fn union_find_sizes_cache(mut position_map: HashMap<(usize, usize), Cell<(usize, usize)>>) -> HashMap<(usize, usize), u128> {
    let mut count_by_root: HashMap<(usize, usize), u128> = HashMap::new();
    let mut c = 0;
    for (_k, v) in position_map.iter() {
        let mut root = v;
        loop{
            c+=1;
            let new_root = position_map.get(&root.get()).unwrap();
            if new_root == root {
                break
            }
            root.set(new_root.get());
        }
        let old_count =  count_by_root.get(&root.get());
        let new_count = old_count.unwrap_or(&0) + 1;
        count_by_root.insert(root.get(), new_count);
    }
    println!("{}", c); //20305
    count_by_root
}

// shorten the path of each parent to the root as we mutate. less iterations, more internal mutability
fn union_find_sizes_cache_common_descendants(mut position_map: HashMap<(usize, usize), Cell<(usize, usize)>>) -> HashMap<(usize, usize), u128> {
    let mut count_by_root: HashMap<(usize, usize), u128> = HashMap::new();
    let mut c = 0;
    for (_k, v) in position_map.iter() {
        let mut root = v;
        let mut to_alter = vec![];
        loop{
            c+=1;
            let new_root = position_map.get(&root.get()).unwrap();
            if new_root == root {
                break
            } else {
                to_alter.push(new_root.get());
            }
            root.set(new_root.get());
        }
        to_alter.into_iter().for_each(|r| {
           position_map.get(&r).unwrap().set(root.get());
        });
        let old_count =  count_by_root.get(&root.get());
        let new_count = old_count.unwrap_or(&0) + 1;
        count_by_root.insert(root.get(), new_count);
    }
    println!("{}", c); // 15913
    count_by_root
}

/*
    This can be modeled as a union-find problem, but also counting the number of elements in each partition/nodegroup

    position_map is now a hashmap of node -> parent node. we want to count the members of each tree, so...
    get a node, get all of its children
    n2 -> n1
    n3 -> n2
    n4 -> n3

    take n3
    n3 -> n2
    n2 -> n1
    count[n1] += 1
    n3 -> n1

    take n4
    n4 -> n3
    n3 -> n1
    count[n1] += 1
     */

fn part_2(map: Vec<Vec<u32>>) -> u128 {
    let mut total = 0;
    let mut position_map = p2_build_links(map);
    // let mut count_by_root = union_find_sizes_cache(position_map.into_iter().map(|(k,v)| (k, Cell::new(v))).collect());
    let mut count_by_root = union_find_sizes_cache_common_descendants(position_map.into_iter().map(|(k,v)| (k, Cell::new(v))).collect());
    // let mut count_by_root = union_find_sizes_clone_keys(position_map);
    let mut xxx = count_by_root.into_values().collect::<Vec<u128>>();
    xxx.sort();
    xxx.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;
    fn test_p1(input: &str, expected: u64) {
        assert_eq!(count_lowest_points(parse(input)), expected);
    }
    fn test_p2(input: &str, expected: u128) {
        assert_eq!(part_2(parse(input)), expected);
    }
    #[test]
    fn parsing_works() {
        let input = "1234";
        assert_eq!(parse(input), vec![vec![1,2,3,4]]);
    }

    #[test]
    fn on_simple_map_works_part_1() {
        test_p1(
          r#"
          1234
          2345
          "#,
            2
        );
    }

    #[test]
    fn on_input_works_part_1() {
        test_p1(&read_to_string("input.txt").unwrap(),
            436
        );
    }

    #[test]
    fn on_simple_map_works_part_2() {
        test_p2(
            r#"
          1294
          2395
          "#,
            8
        );
    }

    #[test]
    fn on_input_works_part_2() {
        test_p2(
            &read_to_string("input.txt").unwrap(),
            1317792
        );
    }
}
