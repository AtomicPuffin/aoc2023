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
    let sequences = do_the_line(input);
    let mut sum = 0;
    for sequence in sequences {
        sum += next_row_end(&sequence);
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let sequences = do_the_line(input);
    let mut sum = 0;
    for sequence in sequences {
        sum += next_row_beginning(&sequence);
    }
    sum
}
fn next_row_end(row: &Vec<i64>) -> i64 {
    let mut next_row = Vec::new();
    let mut a = row[0];
    let mut all_zero = true;
    for n in 1..row.len() {
        let b = row[n];
        next_row.push(b - a);
        if b - a != 0 {
            all_zero = false;
        }
        a = b;
    }
    if all_zero {
        return a;
    } else {
        return next_row_end(&next_row) + a;
    }
}
fn next_row_beginning(row: &Vec<i64>) -> i64 {
    let mut next_row = Vec::new();
    let mut a = row[0];
    let mut all_zero = true;
    for n in 1..row.len() {
        let b = row[n];
        next_row.push(b - a);
        if b - a != 0 {
            all_zero = false;
        }
        a = b;
    }
    if all_zero {
        return a;
    } else {
        return row[0] - next_row_beginning(&next_row);
    }
}

fn do_the_line(input: &str) -> Vec<Vec<i64>> {
    let mut sequences = Vec::new();
    for line in input.lines() {
        let mut sequence = Vec::new();
        for num in line.split(' ') {
            sequence.push(num.parse::<i64>().unwrap());
        }
        sequences.push(sequence);
    }
    sequences
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
        assert_eq!(part_1(&read_file("example.txt")), 114);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1798691765);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1104);
    }
}
