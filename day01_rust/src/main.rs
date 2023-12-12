//use itertools::Itertools;
use std::{char, fs};

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
    let mut sum = 0;
    let mut first: char = '0';
    let mut last: char = '0';
    let mut found_one = false;
    for line in input.lines() {
        for char in line.chars() {
            if char.is_numeric() && !found_one {
                first = char;
                last = char;
                found_one = true;
            } else if char.is_numeric() {
                last = char;
            }
        }
        let num = (first.to_string() + &last.to_string())
            .parse::<i64>()
            .unwrap();
        sum += num;
        found_one = false;
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let _units = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    let mut first: char = '0';
    let mut last: char = '0';
    let mut found_one = false;
    for line in input.lines() {
        let mut newline: String;
        //written numbers can overlap - eigh|t|wo so injection cannot break this
        newline = line.replace("zero", "ze0ro");
        newline = newline.replace("one", "o1ne");
        newline = newline.replace("two", "t2wo");
        newline = newline.replace("three", "th3ree");
        newline = newline.replace("four", "fo4ur");
        newline = newline.replace("five", "f5ve");
        newline = newline.replace("six", "si6x");
        newline = newline.replace("seven", "sev7en");
        newline = newline.replace("eight", "eig8ht");
        newline = newline.replace("nine", "ni9ne");
        for char in newline.chars() {
            if char.is_numeric() && !found_one {
                first = char;
                last = char;
                found_one = true;
            } else if char.is_numeric() {
                last = char;
            }
        }
        let num = (first.to_string() + &last.to_string())
            .parse::<i64>()
            .unwrap();
        sum += num;
        found_one = false;
    }
    sum
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
        assert_eq!(part_1(&read_file("example.txt")), 142);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 55130);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 281);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 54985);
    }
}
