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

fn part_1(input: &str) -> i32 {
    let mut sum = 1;
    let races = do_the_line(input);
    for (time, distance) in races {
        let mut counter = 0;
        for n in 1..time {
            if n * (time - n) > distance {
                counter += 1;
            }
        }
        sum = sum * counter;
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let race = do_the_line2(input);
    let mut counter = 0;
    for n in 1..race.0 {
        if n * (race.0 - n) > race.1 {
            counter += 1;
        }
    }
    counter
}

fn do_the_line(input: &str) -> Vec<(i32, i32)> {
    let re = regex::Regex::new(r" +").unwrap();
    let trim_line = re.replace_all(input, " ");
    let (_times, _distances) = trim_line.split('\n').collect_tuple().unwrap();
    let times = _times.split(' ').collect_vec()[1..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();

    let distances = _distances.split(' ').collect_vec()[1..]
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();
    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push((times[i], distances[i]));
    }
    races
}

fn do_the_line2(input: &str) -> (i64, i64) {
    let re = regex::Regex::new(r" +").unwrap();
    let trim_line = re.replace_all(input, "");
    let (_times_r, _distances_r) = trim_line.split('\n').collect_tuple().unwrap();
    let _time = _times_r.split(':').collect_vec()[1];
    let _distance = _distances_r.split(':').collect_vec()[1];
    let time = _time.parse::<i64>().unwrap();
    let distance = _distance.parse::<i64>().unwrap();
    (time, distance)
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
        assert_eq!(part_1(&read_file("example.txt")), 288);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 503424);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 71503);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 32607562);
    }
}
