#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

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

fn part_1(input: &str) -> i32 {
    todo!()
}

fn part_2(input: &str) -> i32 {
    todo!()
}

fn do_the_line(input: &str) -> (Vec<(i32, (i32, i32, i32))>, Vec<(char, (i32, i32))>) {
    tod!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), todo!());
    }
}