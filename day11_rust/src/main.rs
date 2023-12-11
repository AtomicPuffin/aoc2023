use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"), 2)
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt"), 2));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example.txt"), 100)
    );
    println!(
        "Answer to Part 2: {}",
        part_2(&read_file("input.txt"), 1000000)
    );
}

fn part_1(input: &str, age: i64) -> i64 {
    let space = do_the_line(input, age);
    let mut sum = 0;
    for (r1, c1) in space.clone() {
        for (r2, c2) in space.clone() {
            sum += (r1 - r2).abs() + (c1 - c2).abs();
        }
    }
    sum / 2
}

fn part_2(input: &str, age: i64) -> i64 {
    let space = do_the_line(input, age);
    let mut sum = 0;
    for (r1, c1) in space.clone() {
        for (r2, c2) in space.clone() {
            sum += (r1 - r2).abs() + (c1 - c2).abs();
        }
    }
    sum / 2
}

fn do_the_line(input: &str, age: i64) -> HashSet<(i64, i64)> {
    let mut small_space = HashSet::new();
    let mut row = 0;
    let mut empty_rows = Vec::new();
    let mut max_col = 0;

    //read input and find galaxies, check for empty rows while at it
    for line in input.lines() {
        let mut col = 0;
        let mut empty_row = true;
        for c in line.chars() {
            if c == '#' {
                empty_row = false;
                small_space.insert((row, col));
            }
            col += 1;
        }
        if empty_row {
            empty_rows.push(row);
        }
        max_col = col;
        row += 1;
    }

    let max_row = row;
    let mut empty_cols = Vec::new();

    //iterate all columns and find empty ones
    for c in 0..max_col + 1 {
        let mut empty_col = true;
        for r in 0..max_row + 1 {
            if small_space.contains(&(r, c)) {
                empty_col = false;
            }
        }
        if empty_col {
            empty_cols.push(c);
        }
    }

    //calculate expanded space by checking how many rows/cols are empty before those coordinates
    //note that expansion by 10 actually means 9 extra + the one already there
    let mut space = HashSet::new();
    for (r, c) in small_space {
        let mut empty_rcount = 0;
        let mut empty_ccount = 0;
        for er in &empty_rows {
            if er < &r {
                empty_rcount += 1;
            }
        }
        for ec in &empty_cols {
            if ec < &c {
                empty_ccount += 1;
            }
        }
        space.insert((
            r + age * empty_rcount - empty_rcount,
            c + age * empty_ccount - empty_ccount,
        ));
    }
    space
}

/*fn print_space(space: &HashMap<(i32, i32), char>, max_row: i32, max_col: i32) {
    for row in 0..max_row + 1 {
        for col in 0..max_col + 1 {
            if space.contains_key(&(row, col)) {
                print!("{}", space[&(row, col)]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}*/

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt"), 2), 374);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), 2), 10292708);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt"), 100), 8410);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), 1000000), 790194712336);
    }
}
