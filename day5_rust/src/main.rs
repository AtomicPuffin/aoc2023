#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use itertools::{Itertools, MapResults};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::MAIN_SEPARATOR_STR;
use std::{fs, i32};

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
    let mut lowest = i64::MAX;
    let (seeds, sections) = do_the_line(input);
    for mut seed in seeds {
        for sec in sections.iter() {
            for row in sec.iter() {
                if seed >= row[1] && seed <= row[1] + row[2] {
                    seed += row[0] - row[1];
                    break;
                }
            }
        }
        if seed < lowest {
            lowest = seed;
        }
    }

    lowest
}

fn part_2(input: &str) -> i64 {
    // brute force, run with --release
    let mut lowest = i64::MAX;
    let (seeds, sections) = do_the_line(input);
    let mut _pairs = seeds.iter().peekable();
    while _pairs.len() > 0 {
        let pair = (_pairs.next().unwrap(), _pairs.next().unwrap());
        for mut seed in *pair.0..(pair.0 + pair.1 - 1) {
            for sec in sections.iter() {
                for row in sec.iter() {
                    if seed >= row[1] && seed <= row[1] + row[2] - 1 {
                        seed += row[0] - row[1];
                        break;
                    }
                }
            }
            if seed < lowest {
                lowest = seed;
            }
        }
    }
    lowest
}

fn do_the_line(input: &str) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let _sections = input.split("\n\n").collect_vec();
    let _seeds = _sections[0].split(' ').collect_vec()[1..].to_vec();
    let mut sections = Vec::new();
    //dest range start,  source range start,  range length

    let mut seeds = Vec::new();
    for s in _seeds {
        //println!("seed: {}", s);
        seeds.push(s.parse::<i64>().unwrap());
    }
    for sec in _sections {
        let mut section = Vec::new();
        for line in sec.lines().skip(1) {
            let nums = line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec();
            section.push(nums);
        }
        sections.push(section);
    }

    (seeds, sections)
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt")), 35);
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 462648396);
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 46);
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 2520479);
    }
}
