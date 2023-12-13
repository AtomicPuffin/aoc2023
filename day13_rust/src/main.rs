use itertools::Itertools;
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
    let patterns = do_the_line(input);
    let mut sum = 0;
    for pattern in &patterns {
        let rows = find_horizontal(&pattern, 0);
        let columns = find_vertical(&pattern, 0);
        if rows != -1 {
            sum += 100 * (rows + 1);
        }
        if columns != -1 {
            sum += columns + 1;
        }
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let patterns = do_the_line(input);
    let mut sum = 0;
    for pattern in patterns.clone() {
        let row = find_horizontal(&pattern, 1);
        let columns = find_vertical(&pattern, 1);
        if row != -1 {
            sum += 100 * (row + 1);
        }
        if columns != -1 {
            sum += columns + 1;
        }
    }
    sum
}

fn find_horizontal(pattern: &(HashSet<(i64, i64)>, i64, i64), max_difference: i64) -> i64 {
    for y in 0..pattern.1 - 1 {
        let mut differences = 0;
        let mut offset = 0;

        'row: loop {
            if offset > y || offset >= pattern.1 - y - 1 {
                break;
            }
            for x in 0..pattern.2 {
                if pattern.0.contains(&(y - offset, x)) && pattern.0.contains(&(y + 1 + offset, x))
                {
                    continue;
                } else if !pattern.0.contains(&(y - offset, x))
                    && !pattern.0.contains(&(y + 1 + offset, x))
                {
                    continue;
                } else {
                    differences += 1;
                    if differences > max_difference {
                        break 'row;
                    }
                }
            }
            offset += 1;
        }
        if differences == max_difference {
            println!("{} {}", y, differences);
            return y;
        }
    }
    -1
}

fn find_vertical(pattern: &(HashSet<(i64, i64)>, i64, i64), difference: i64) -> i64 {
    for x in 0..pattern.2 - 1 {
        let mut differences = 0;
        let mut offset = 0;

        loop {
            if offset > x || offset >= pattern.2 - x - 1 {
                break;
            }
            for y in 0..pattern.1 {
                if pattern.0.contains(&(y, x - offset)) && pattern.0.contains(&(y, x + 1 + offset))
                {
                    continue;
                } else if !pattern.0.contains(&(y, x - offset))
                    && !pattern.0.contains(&(y, x + 1 + offset))
                {
                    continue;
                } else {
                    differences += 1;
                    if differences > difference {
                        break;
                    }
                }
            }
            offset += 1;
        }
        if differences == difference {
            return x;
        }
    }

    -1
}

fn do_the_line(input: &str) -> Vec<(HashSet<(i64, i64)>, i64, i64)> {
    let pats = input.split("\n\n").collect_vec();
    let mut patterns = Vec::new();
    for p in pats {
        let mut pattern = HashSet::new();
        let mut row: i64 = 0;
        let mut col: i64 = 0;
        for line in p.lines() {
            col = 0;
            for c in line.chars() {
                if c == '#' {
                    pattern.insert((row, col));
                }
                col += 1;
            }
            row += 1;
        }
        patterns.push((pattern.clone(), row, col));
    }
    patterns
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
        assert_eq!(part_1(&read_file("example.txt")), 405);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 34202);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 400);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 34230);
    }
}
