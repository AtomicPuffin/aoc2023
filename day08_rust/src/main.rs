use core::panic;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example2.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let current = "AAA".to_string();
    let (inst, network_tree) = do_the_line(input);
    let mut ends = HashSet::new();
    let end = "ZZZ".to_string();
    ends.insert(&end);

    get_end_cycle(current, &network_tree, &ends, &inst)
}

fn part_2(input: &str) -> i64 {
    let mut starts: Vec<String> = Vec::new();
    let (inst, network_tree) = do_the_line(input);
    for key in network_tree.keys() {
        if key.chars().collect_vec()[2] == 'A' {
            starts.push(key.to_string());
        }
    }
    let mut ends = HashSet::new();
    for key in network_tree.keys() {
        if key.chars().collect_vec()[2] == 'Z' {
            ends.insert(key);
        }
    }

    let mut cycles = Vec::new();
    for current in starts {
        cycles.push(get_end_cycle(current, &network_tree, &ends, &inst));
    }

    let mut ans = lcm(cycles[0], cycles[1]);
    if cycles.len() > 2 {
        for i in 2..cycles.len() {
            ans = lcm(ans, cycles[i]);
        }
    }
    ans
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
fn gcd(a: i64, b: i64) -> i64 {
    let mut n = a.max(b);
    let mut d = a.min(b);
    while d != 0 {
        let rem = n % d;
        n = d;
        d = rem;
    }
    n
}
fn get_end_cycle(
    start: String,
    network_tree: &HashMap<String, (String, String)>,
    ends: &HashSet<&String>,
    inst: &String,
) -> i64 {
    let mut current = start.clone();
    let mut counter = 0;
    loop {
        for c in inst.chars() {
            //println!("{}: {}", counter, current);
            if ends.contains(&current) {
                return counter;
            }

            match c {
                'L' => {
                    current = network_tree.get(&current).unwrap().0.to_string();
                }
                'R' => {
                    current = network_tree.get(&current).unwrap().1.to_string();
                }
                _ => panic!("Unknown instruction"),
            }
            counter += 1;
        }
    }
}

fn do_the_line(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (inst, lines) = input.split_once("\n\n").unwrap();

    let mut network_tree = HashMap::new();
    for line in lines.lines() {
        let words = line.split(' ').collect_vec();
        let a = words[2][1..4].to_string();
        let b = words[3][0..3].to_string();
        network_tree.insert(words[0].to_string(), (a, b));
    }

    (inst.to_string(), network_tree)
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
        assert_eq!(part_1(&read_file("example.txt")), 2);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 12737);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 6);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 9064949303801);
    }
}
