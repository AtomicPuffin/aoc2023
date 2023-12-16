use itertools::Itertools;

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
    let sequence = do_the_line(input);
    let mut sum = 0;
    for seq in sequence {
        let mut hash = 0;
        for c in seq.chars() {
            hash = (hash + c as i64) * 17 % 256;
        }
        sum += hash;
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let sequence = do_the_line(input);
    let mut sum = 0;
    let mut boxes: Vec<Vec<(String, i64)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for seq in sequence {
        let mut hash = 0;
        let mut label = "".to_string();
        let mut sq = seq.chars();
        while let Some(c) = sq.next() {
            if c == '-' {
                //remove remove label from box
                // maintain queu
                boxes[hash as usize].retain(|x| x.0 != label);
            } else if c == '=' {
                //if label exist, replace
                //otherwise push to end
                let mut exists = false;
                for lense in 0..boxes[hash as usize].len() {
                    if boxes[hash as usize][lense].0 == label {
                        exists = true;
                        boxes[hash as usize][lense] = (
                            label.to_string(),
                            sq.next().unwrap().to_digit(10).unwrap() as i64,
                        );
                    }
                }
                if !exists {
                    boxes[hash as usize].push((
                        label.clone(),
                        sq.next().unwrap().to_digit(10).unwrap() as i64,
                    ));
                }
            } else {
                label += &c.to_string();
                hash = (hash + c as i64) * 17 % 256;
            }
        }
    }
    //add focusing powers - 1+ box id * slot starting at 1 * focal length
    for n in 0..256 {
        for boxe in 0..boxes[n].len() {
            sum += (n + 1) as i64 * (boxe as i64 + 1) * boxes[n][boxe as usize].1;
        }
    }

    sum
}

fn do_the_line(input: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\n").unwrap();
    let trim_line = re.replace_all(input, "");
    let sequence = trim_line.split(',').map(|x| x.to_string()).collect_vec();
    sequence
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
        assert_eq!(part_1(&read_file("example.txt")), 1320);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 517965);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 145);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 267372);
    }
}
