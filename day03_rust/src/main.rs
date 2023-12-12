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
    let (numbers, symbols) = do_the_line(input);
    let mut sum = 0;
    for (num, (row, start, stop)) in numbers {
        for (_, (srow, scol)) in &symbols {
            if row == srow + 1 || row == srow - 1 || row == *srow {
                if start <= scol + 1 && stop >= scol - 1 {
                    sum += num;
                }
            }
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let (numbers, symbols) = do_the_line(input);
    let mut sum = 0;
    for (symbol, (srow, scol)) in &symbols {
        let mut adjacent: Vec<i32> = Vec::new();
        if *symbol == '*' {
            for (num, (row, start, stop)) in numbers.clone() {
                if row == srow + 1 || row == srow - 1 || row == *srow {
                    if start <= scol + 1 && stop >= scol - 1 {
                        adjacent.push(num);
                    }
                }
            }
            if adjacent.len() == 2 {
                sum += adjacent[0] * adjacent[1];
            }
        }
    }
    sum
}

fn do_the_line(input: &str) -> (Vec<(i32, (i32, i32, i32))>, Vec<(char, (i32, i32))>) {
    //iterate rows, find numbers, add them as (num, (start col, stop col, row))
    //also add (symbol, (col, row)
    let mut numbers: Vec<(i32, (i32, i32, i32))> = Vec::new();
    let mut symbols: Vec<(char, (i32, i32))> = Vec::new();
    let mut row = 0;
    for line in input.lines() {
        let mut column = 0;
        let mut word: Vec<char> = Vec::new();
        for char in line.chars() {
            if char == '.' {
                //pass
            } else if char.is_numeric() {
                word.push(char);
                column += 1;
                //catch end of line
                if column < line.chars().count() as i32 {
                    continue;
                }
            } else {
                symbols.push((char, (row, column)));
            }
            if word.len() > 0 {
                let num = word.iter().collect::<String>().parse::<i32>().unwrap();
                numbers.push((num, (row, column - word.len() as i32, column - 1)));
                word = Vec::new();
            }
            column += 1;
        }
        row += 1;
    }
    (numbers, symbols)
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
        assert_eq!(part_1(&read_file("example.txt")), 4361);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 553079);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 467835);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 84363105);
    }
}
