use core::panic;
use itertools::Itertools;
use std::collections::HashMap;

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
    let (inst, parts) = do_the_line(input);
    let mut sum = 0;
    for part in parts {
        let mut current = "in".to_string();
        loop {
            if current == "A" {
                sum += part[0] + part[1] + part[2] + part[3];
                break;
            } else if current == "R" {
                break;
            }
            for rule in inst.get(&current).unwrap() {
                if rule.0 == 4 {
                    current = rule.3.to_string();
                    break;
                }
                match rule.1.as_str() {
                    "<" => {
                        if part[rule.0] < rule.2 {
                            current = rule.3.to_string();
                            break;
                        }
                    }
                    ">" => {
                        if part[rule.0] > rule.2 {
                            current = rule.3.to_string();
                            break;
                        }
                    }
                    _ => {
                        panic!("Unknown operator")
                    }
                }
            }
        }
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let (inst, _) = do_the_line(input);
    let part = [(1, 4000); 4];
    let current = "in".to_string();
    recurse(&inst, part, current)
}

fn recurse(
    inst: &HashMap<String, Vec<(usize, String, i64, String)>>,
    mut part: [(i64, i64); 4],
    mut current: String,
) -> i64 {
    let mut sum = 0;
    loop {
        if current == "A" {
            return (part[0].1 - part[0].0 + 1)
                * (part[1].1 - part[1].0 + 1)
                * (part[2].1 - part[2].0 + 1)
                * (part[3].1 - part[3].0 + 1)
                + sum;
        } else if current == "R" {
            return sum;
        }
        for rule in inst.get(&current).unwrap() {
            if rule.0 == 4 {
                current = rule.3.to_string();
                break;
            }
            match rule.1.as_str() {
                "<" => {
                    if part[rule.0].1 < rule.2 {
                        // all smaller, jump
                        current = rule.3.to_string();
                        break;
                    } else if part[rule.0].0 > rule.2 {
                        // all larger, continue
                    } else {
                        // split
                        let mut part_split = part.clone();
                        part_split[rule.0] = (part[rule.0].0, rule.2 - 1);
                        part[rule.0] = (rule.2, part[rule.0].1);
                        sum += recurse(inst, part_split, rule.3.to_string());
                        //part will continue here
                    }
                }
                ">" => {
                    if part[rule.0].0 > rule.2 {
                        //all larger, jump
                        current = rule.3.to_string();
                        break;
                    } else if part[rule.0].1 < rule.2 {
                        // all smaller, continue
                    } else {
                        // split
                        let mut part_split = part.clone();
                        part_split[rule.0] = (rule.2 + 1, part[rule.0].1);
                        part[rule.0] = (part[rule.0].0, rule.2);
                        sum += recurse(inst, part_split, rule.3.to_string());
                        //part will continue here
                    }
                }
                _ => {
                    panic!("Unknown operator")
                }
            }
        }
    }
}

fn do_the_line(
    input: &str,
) -> (
    HashMap<String, Vec<(usize, String, i64, String)>>,
    Vec<[i64; 4]>,
) {
    let (i, p) = input.split("\n\n").collect_tuple().unwrap();
    let mut inst = HashMap::new();
    for l in i.lines() {
        let (head, tail) = l.split("{").collect_tuple().unwrap();
        let re = regex::Regex::new(r"}").unwrap();
        let trim_line = re.replace_all(tail, "");
        let mut rules = Vec::new();
        for rule in trim_line.split(",").collect_vec() {
            if rule.contains(":") {
                let c = &rule[0..1];
                let mut cat: usize = 5;
                match c {
                    "x" => cat = 0,
                    "m" => cat = 1,
                    "a" => cat = 2,
                    "s" => cat = 3,
                    _ => (),
                }
                let op = rule[1..2].to_string();
                let (v, targ) = rule[2..].split(":").collect_tuple().unwrap();
                let val = v.parse::<i64>().unwrap();
                rules.push((cat, op, val, targ.to_string()));
            } else {
                // use 4 to identify rule-less forward
                rules.push((4, "".to_string(), 0, rule.to_string()));
            }
        }
        inst.insert(head.to_string(), rules);
    }

    let mut parts = Vec::new();
    for part in p.lines() {
        let cat = part.split(",").collect_vec();
        let x = cat[0][3..].parse::<i64>().unwrap();
        let m = cat[1][2..].parse::<i64>().unwrap();
        let a = cat[2][2..].parse::<i64>().unwrap();
        let s = cat[3][2..cat[3].len() - 1].parse::<i64>().unwrap();
        let p = [x, m, a, s];
        parts.push(p);
    }

    (inst, parts)
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
        assert_eq!(part_1(&read_file("example.txt")), 19114);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 263678);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 167409079868000);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 125455345557345);
    }
}
