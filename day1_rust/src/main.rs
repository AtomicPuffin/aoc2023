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
    todo!()
}

fn part_2(input: &str) -> i64 {
    //let lines: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    todo!()
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
        assert_eq!(part_1(&read_file("example.txt")), todo!());
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), todo!());
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), todo!());
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), todo!());
    }
}
