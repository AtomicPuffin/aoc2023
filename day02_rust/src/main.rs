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

fn part_1(input: &str) -> i32 {
    let mut limiter = HashMap::new();
    limiter.insert("red", 12);
    limiter.insert("green", 13);
    limiter.insert("blue", 14);
    let mut sum = 0;

    for line in input.lines() {
        let re = regex::Regex::new(r", |: |; ").unwrap();
        let mut is_ok = true;
        let mut game_id = 0;
        for pair in re.split(line) {
            let tuple: Vec<&str> = pair.split(' ').collect();
            if tuple[0] == "Game" {
                game_id = tuple[1].parse::<i32>().unwrap();
                continue;
            }
            let colour = tuple[1];
            let count = tuple[0].parse::<i32>().unwrap();
            if limiter[colour] < count {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            sum += game_id;
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let mut max_colour = HashMap::new();
    let mut sum = 0;

    for line in input.lines() {
        max_colour.insert("red", 0);
        max_colour.insert("green", 0);
        max_colour.insert("blue", 0);

        let re = regex::Regex::new(r", |: |; ").unwrap();

        for pair in re.split(line) {
            let tuple: Vec<&str> = pair.split(' ').collect();
            if tuple[0] == "Game" {
                continue;
            }
            let colour = tuple[1];
            let count = tuple[0].parse::<i32>().unwrap();
            if max_colour[colour] < count {
                max_colour.insert(colour, count);
            }
        }
        sum += max_colour["red"] * max_colour["green"] * max_colour["blue"];
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
        assert_eq!(part_1(&read_file("example.txt")), 8);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1734);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 2286);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 70387);
    }
}
