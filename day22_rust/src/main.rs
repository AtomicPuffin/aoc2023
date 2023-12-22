use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let mut bricks = do_the_line(input);

    //z start is always smaller than z stop
    let resting_bricks = find_resting(&mut bricks);

    let mut immovable = HashSet::new();
    for brick in resting_bricks.clone() {
        let mut rest_on = Vec::new();
        for rb in resting_bricks.iter() {
            if rb.0 .0 > brick.1 .0
                || rb.1 .0 < brick.0 .0
                || rb.0 .1 > brick.1 .1
                || rb.1 .1 < brick.0 .1
            {
                //no overlap move along
                continue;
            } else {
                //overlap, test resting
                if rb.1 .2 == brick.0 .2 - 1 {
                    //resting
                    rest_on.push(rb);
                }
            }
        }
        if rest_on.len() == 1 {
            immovable.insert(rest_on[0]);
        }
    }

    resting_bricks.len() as i64 - immovable.len() as i64
}

fn part_2(input: &str) -> i64 {
    let mut bricks = do_the_line(input);

    //z start is always smaller than z stop
    let resting_bricks = find_resting(&mut bricks);

    let mut brick_tree = HashMap::new();

    for brick in resting_bricks.clone() {
        brick_tree.insert(brick, (HashSet::new(), HashSet::new()));
    }
    for brick in resting_bricks.clone() {
        for rb in resting_bricks.iter() {
            if rb.0 .0 > brick.1 .0
                || rb.1 .0 < brick.0 .0
                || rb.0 .1 > brick.1 .1
                || rb.1 .1 < brick.0 .1
            {
                //no overlap move along
                continue;
            } else {
                //overlap, test resting
                if rb.1 .2 == brick.0 .2 - 1 {
                    //resting
                    brick_tree.get_mut(&brick).unwrap().0.insert(rb);
                    brick_tree.get_mut(rb).unwrap().1.insert(brick);
                }
            }
        }
    }
    let mut sum = 0;
    for b in brick_tree.keys() {
        let mut count = 0;
        let mut temp_tree = brick_tree.clone();
        let mut queue = VecDeque::new();
        queue.push_back(b.clone());
        while let Some(brick) = queue.pop_front() {
            count += 1;
            for supported_b in temp_tree.get(&brick).unwrap().1.clone() {
                temp_tree.get_mut(&supported_b).unwrap().0.remove(&brick);
                if temp_tree.get(&supported_b).unwrap().0.is_empty() {
                    queue.push_back(supported_b.clone());
                }
            }
        }
        sum += count - 1;
    }
    sum
}

fn find_resting(
    bricks: &mut Vec<((i64, i64, i64), (i64, i64, i64))>,
) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    bricks.sort_by(|a, b| b.0 .2.cmp(&a.0 .2));
    let mut resting_bricks = Vec::new();
    while let Some(brick) = bricks.pop() {
        if brick.0 .2 == 1 {
            //on floor
            resting_bricks.push(brick);
        } else {
            //find first collision when falling
            let mut found_floor = true;
            resting_bricks.sort_by(|a, b| b.1 .2.cmp(&a.1 .2));

            for rb in resting_bricks.iter() {
                if rb.0 .0 > brick.1 .0
                    || rb.1 .0 < brick.0 .0
                    || rb.0 .1 > brick.1 .1
                    || rb.1 .1 < brick.0 .1
                {
                    //no collision, keep looking
                    continue;
                } else {
                    //collision
                    let new_z_start = rb.1 .2 + 1;
                    let new_z_stop = new_z_start + brick.1 .2 - brick.0 .2;
                    let new_brick = (
                        (brick.0 .0, brick.0 .1, new_z_start),
                        (brick.1 .0, brick.1 .1, new_z_stop),
                    );
                    found_floor = false;
                    resting_bricks.push(new_brick);
                    break;
                }
            }
            if found_floor {
                //found floor
                let new_brick = (
                    (brick.0 .0, brick.0 .1, 1),
                    (brick.1 .0, brick.1 .1, 1 + brick.1 .2 - brick.0 .2),
                );
                resting_bricks.push(new_brick);
            }
        }
    }
    resting_bricks
}

fn do_the_line(input: &str) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    let mut bricks = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once('~').unwrap();
        let sa = a
            .split(',')
            .collect_vec()
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        let sb = b
            .split(',')
            .collect_vec()
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        let start = (sa[0], sa[1], sa[2]);
        let stop = (sb[0], sb[1], sb[2]);
        bricks.push((start, stop));
    }

    bricks
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt")), 5);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 527);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 7);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 100376);
    }
}
