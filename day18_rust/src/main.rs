use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
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
    let dig_plan = do_the_line(input);

    let mut current = (0, 0);
    let mut trench = HashSet::new();
    let mut max = (0, 0);
    let mut min = (i64::MAX, i64::MAX);
    let mut sum = 0;
    for (direction, distance) in dig_plan {
        for _ in 0..distance {
            match direction.as_str() {
                "U" => current.0 -= 1,
                "D" => current.0 += 1,
                "R" => current.1 += 1,
                "L" => current.1 -= 1,
                _ => panic!("Unknown direction"),
            }
            trench.insert(current);
            sum += 1;
            if max.0 < current.0 {
                max.0 = current.0;
            }
            if max.1 < current.1 {
                max.1 = current.1;
            }
            if min.0 > current.0 {
                min.0 = current.0;
            }
            if min.1 > current.1 {
                min.1 = current.1;
            }
        }
    }
    println!("Max: {:?}", max);

    let mut filled = HashMap::new();
    let mut queue = Vec::new();

    for y in min.0..max.0 + 1 {
        for x in min.0..max.1 + 1 {
            if !trench.contains(&(y, x)) && !filled.contains_key(&(y, x)) {
                queue.push((y, x));
                //println!("start {} {}", y, x);
                let mut inside = true;
                let mut surface = HashSet::new();
                while let Some((y, x)) = queue.pop() {
                    //println!(" {} {}", y, x);
                    if trench.contains(&(y, x)) {
                        continue;
                    }
                    if y <= min.0 || x <= min.1 || y >= max.0 || x >= max.1 {
                        inside = false;
                        continue;
                    }
                    if surface.contains(&(y, x)) {
                        continue;
                    }
                    surface.insert((y, x));
                    if !surface.contains(&(y - 1, x)) {
                        queue.push((y - 1, x));
                    }
                    if !surface.contains(&(y + 1, x)) {
                        queue.push((y + 1, x));
                    }
                    if !surface.contains(&(y, x - 1)) {
                        queue.push((y, x - 1));
                    }
                    if !surface.contains(&(y, x + 1)) {
                        queue.push((y, x + 1));
                    }
                }
                for (y, x) in surface {
                    filled.insert((y, x), inside);
                }
            }
        }
    }

    for (_, i) in filled.clone() {
        if i {
            sum += 1;
        }
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let dig_plan = do_the_line2(input);
    let mut current: (i64, i64) = (0, 0);
    let mut min = i64::MAX;
    let mut sum = 0;
    let mut corners = Vec::new();

    for (direction, distance) in dig_plan {
        match direction {
            3 => current.0 -= distance,
            1 => current.0 += distance,
            0 => current.1 += distance,
            2 => current.1 -= distance,
            _ => panic!("Unknown direction"),
        }
        corners.push(current);
        if min > current.0 {
            min = current.0;
        }
    }
    corners.sort();
    corners.reverse();

    let mut active: Vec<(i64, i64)> = Vec::new();
    let mut last_y = min;

    while let Some(start) = corners.pop() {
        let end = corners.pop().unwrap();
        if end.0 != start.0 {
            println!("strt {} {} stop {} {}", start.0, start.1, end.0, end.1);
            panic!("Not a line");
        }

        let mut flick_on = true;
        let mut double = false;
        for act in active.clone() {
            //add surface to sum from last corner row to this row
            sum += (start.0 - last_y) * (act.1 - act.0 + 1);
            if act.0 == start.1 && act.1 == end.1 {
                //exact match, remove active
                active.retain(|&x| x != act);
                flick_on = false;
            } else if act.1 == start.1 {
                //sides touch, extend right
                if double {
                    continue;
                }
                let mut new_end = (false, 0);
                //check if it fills a gap, if so remove both and ignore other on future iteration
                for check in active.clone() {
                    if check.0 == end.1 {
                        active.retain(|&x| x != check);
                        new_end = (true, check.1);
                        double = true;
                    }
                }
                active.retain(|&x| x != act);
                if new_end.0 {
                    active.push((act.0, new_end.1));
                    //add new part of this row to sum
                    sum += end.1 - start.1 - 1;
                } else {
                    active.push((act.0, end.1));
                    //add new part of this row to sum
                    sum += end.1 - start.1;
                }
                flick_on = false;
            } else if act.0 == end.1 {
                //extend left
                if double {
                    continue;
                }
                let mut new_start = (false, 0);
                for check in active.clone() {
                    if check.1 == start.1 {
                        active.retain(|&x| x != check);
                        new_start = (true, check.0);
                        double = true;
                    }
                }
                active.retain(|&x| x != act);
                if new_start.0 {
                    active.push((new_start.1, act.1));
                    sum += end.1 - start.1 - 1;
                } else {
                    active.push((start.1, act.1));
                    sum += end.1 - start.1;
                }
                flick_on = false;
            } else if act.0 <= start.1 && act.1 >= end.1 {
                // inside hit

                if act.0 == start.1 {
                    // right remainder
                    active.retain(|&x| x != act);
                    active.push((end.1, act.1));
                } else if act.1 == end.1 {
                    // left remainder
                    active.retain(|&x| x != act);
                    active.push((act.0, start.1));
                } else {
                    // middle, split and remove middle
                    active.retain(|&x| x != act);
                    active.push((act.0, start.1));
                    active.push((end.1, act.1));
                }
                flick_on = false;
            } else if act.0 >= start.1 && act.1 <= end.1 {
                // outside, remove all but leave flick_on, we add larger later
                active.retain(|&x| x != act);
                //subtract already added part of row
                sum -= act.1 - act.0 + 1;
            }
        }
        if flick_on {
            active.push((start.1, end.1));
            sum += end.1 - start.1 + 1
        }
        last_y = start.0;
    }

    sum
}

fn do_the_line(input: &str) -> Vec<(String, i64)> {
    let mut dig_plan = Vec::new();
    for line in input.lines() {
        let row = line.split(' ').collect_vec();
        let direction = row[0].to_string();
        let distance = row[1].parse::<i64>().unwrap();
        dig_plan.push((direction, distance));
    }
    dig_plan
}

fn do_the_line2(input: &str) -> Vec<(i64, i64)> {
    let mut dig_plan = Vec::new();
    for line in input.lines() {
        let row = line.split(' ').collect_vec();
        let distance = i64::from_str_radix(&row[2][2..7], 16).unwrap();
        let direction = row[2][7..8].parse::<i64>().unwrap();
        dig_plan.push((direction, distance));
    }
    dig_plan
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
        assert_eq!(part_1(&read_file("example.txt")), 62);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 70026);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 952408144115);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 68548301037382);
    }
}
