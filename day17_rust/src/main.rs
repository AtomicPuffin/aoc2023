use std::collections::BinaryHeap;
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

fn part_1(input: &str) -> i64 {
    let city = do_the_line(input);

    let board_size = (
        city.keys().map(|x| x.0).max().unwrap(),
        city.keys().map(|x| x.1).max().unwrap(),
    );

    let min = djikstra(&city, &board_size, 0, 3);

    min
}

fn part_2(input: &str) -> i64 {
    let city = do_the_line(input);

    let board_size = (
        city.keys().map(|x| x.0).max().unwrap(),
        city.keys().map(|x| x.1).max().unwrap(),
    );

    let min = djikstra(&city, &board_size, 4, 10);

    min
}

fn djikstra(city: &HashMap<(i64, i64), i64>, board_size: &(i64, i64), min: i64, max: i64) -> i64 {
    let mut losses: HashMap<(i64, i64), HashMap<(i64, i64, i64), i64>> = HashMap::new();

    for y in 0..board_size.0 + 1 {
        for x in 0..board_size.1 + 1 {
            losses.insert((y, x), HashMap::new());
        }
    }
    let mut start = HashMap::new();
    start.insert((0, 1, 1), 0);
    start.insert((-1, 0, 1), 0);
    losses.insert((0, 0), start);

    let mut heap = BinaryHeap::new();

    heap.push((0, 0));

    while let Some(current) = heap.pop() {
        for (crossing, loss) in losses.get(&current).unwrap().clone() {
            for direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if crossing.0 == -direction.0 && crossing.1 == -direction.1 {
                    continue;
                }
                let next_pos = (current.0 + direction.0, current.1 + direction.1);
                if losses.contains_key(&next_pos) {
                    let mut next_crossing = (direction.0, direction.1, crossing.2);
                    let next_loss = loss + city.get(&next_pos).unwrap();

                    if crossing.0 == direction.0 && crossing.1 == direction.1 {
                        next_crossing.2 = crossing.2 + 1;
                        if next_crossing.2 > max {
                            continue;
                        }
                    } else {
                        if next_crossing.2 < min {
                            continue;
                        }
                        next_crossing.2 = 1;
                    }
                    let mut iterate = true;

                    if losses[&next_pos].contains_key(&next_crossing) {
                        if !(losses[&next_pos][&next_crossing] > next_loss) {
                            iterate = false;
                        }
                    }
                    if iterate {
                        losses
                            .get_mut(&next_pos)
                            .unwrap()
                            .insert(next_crossing, next_loss);
                        heap.push(next_pos);
                    }
                }
            }
        }
    }
    let mut result = i64::MAX;
    for (c, v) in losses[&(board_size.0, board_size.1)].clone() {
        if c.2 >= min {
            if v < result {
                result = v;
            }
        }
    }
    result
}

fn do_the_line(input: &str) -> HashMap<(i64, i64), i64> {
    let mut row: i64 = 0;
    let mut col;
    let mut city = HashMap::new();
    for line in input.lines() {
        col = 0;
        for c in line.chars() {
            city.insert((row, col), c.to_digit(10).unwrap() as i64);
            col += 1;
        }
        row += 1;
    }
    city
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
        assert_eq!(part_1(&read_file("example.txt")), 102);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 814);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 94);
    }
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 974);
    }
}
