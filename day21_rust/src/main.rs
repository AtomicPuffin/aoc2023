use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"), 6)
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt"), 64));
    // due to lack of central canal this works differently, easier to ignore
    /*println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example.txt"), 5000)
    );*/
    println!(
        "Answer to Part 2: {}",
        part_2(&read_file("input.txt"), 26501365)
    );
}

fn part_1(input: &str, steps: i64) -> i64 {
    let (garden, start, max) = do_the_line(input);
    let counts = calc_odd_even(&garden, max, start);
    if steps % 2 == 0 {
        return counts.get(&steps).unwrap().1;
    } else {
        return counts.get(&steps).unwrap().0;
    }
}

fn part_2(input: &str, steps: i64) -> i64 {
    // S is in the absolute center with straight empty canals in all directions
    // propagation will happen along canals and then saturate everything except edges
    // directly ESWN will be reached by central canal, everyone else from exactly one corner
    // calculate saturated states (they will alternate between two states mirroring eachother where every other box has the other state)
    // calculate numbers from each corner or central line until saturated once
    // calculate number of saturated cubes, and find all edge cubes and sum their remaining steps
    // due to lazy only works on odd numbers of steps, for even all .0 and .1 should be swapped

    // example works differently due to assymetric shape resulting in variations in channels and is ignored
    let (garden, start, max) = do_the_line(input);
    let mut directions = HashMap::new();
    let first = calc_odd_even(&garden, max, start);
    let dirs = vec![
        ("E", (65, 0)),
        ("S", (0, 65)),
        ("W", (65, 130)),
        ("N", (130, 65)),
        ("NE", (0, 130)),
        ("SE", (130, 130)),
        ("SW", (130, 0)),
        ("NW", (0, 0)),
    ];
    for d in dirs.clone() {
        directions.insert(d.0, calc_odd_even(&garden, max, d.1));
    }
    let mut sum = 0;
    let diag = steps - 132;
    let straight = steps - 66;
    // we will skip origo
    sum += first.get(&262).unwrap().0;
    for x in 0..(straight / (131) + 1) {
        //do central line E W
        if x * 131 < straight - 262 {
            if x % 2 == 0 {
                sum += 7421 * 4;
            } else {
                sum += 7450 * 4;
            }
        } else {
            let temp1 = directions
                .get(&"N")
                .unwrap()
                .get(&(straight - x * 131))
                .unwrap();
            let temp2 = directions
                .get(&"S")
                .unwrap()
                .get(&(straight - x * 131))
                .unwrap();
            let temp3 = directions
                .get(&"E")
                .unwrap()
                .get(&(straight - x * 131))
                .unwrap();
            let temp4 = directions
                .get(&"W")
                .unwrap()
                .get(&(straight - x * 131))
                .unwrap();
            if x % 2 == 0 {
                sum += temp1.0;
                sum += temp2.0;
                sum += temp3.0;
                sum += temp4.0;
            } else {
                sum += temp1.1;
                sum += temp2.1;
                sum += temp3.1;
                sum += temp4.1;
            }
        }
    }
    for y in 0..(diag / (131) + 1) {
        for x in 0..(diag / (131) + 1) {
            // all corners are the same except on edges
            // where propagation is different within a box depending on direction
            // so we do one quadrant only
            if x * 131 + y * 131 > diag {
                continue;
            } else if y * 131 + x * 131 < diag - 262 {
                if (y + x) % 2 == 0 {
                    sum += 7421 * 4;
                } else {
                    sum += 7450 * 4;
                }
            } else {
                let temp1 = directions
                    .get(&"NE")
                    .unwrap()
                    .get(&(diag - (y + x) * 131))
                    .unwrap();
                let temp2 = directions
                    .get(&"NW")
                    .unwrap()
                    .get(&(diag - (y + x) * 131))
                    .unwrap();
                let temp3 = directions
                    .get(&"SE")
                    .unwrap()
                    .get(&(diag - (y + x) * 131))
                    .unwrap();
                let temp4 = directions
                    .get(&"SW")
                    .unwrap()
                    .get(&(diag - (y + x) * 131))
                    .unwrap();
                if (x + y) % 2 == 0 {
                    sum += temp1.0;
                    sum += temp2.0;
                    sum += temp3.0;
                    sum += temp4.0;
                } else {
                    sum += temp1.1;
                    sum += temp2.1;
                    sum += temp3.1;
                    sum += temp4.1;
                }
            }
        }
    }

    sum
}

fn calc_odd_even(
    garden: &HashSet<(i64, i64)>,
    max: (i64, i64),
    start: (i64, i64),
) -> HashMap<i64, (i64, i64)> {
    //assumes start is even, flip if not
    let mut counters = HashMap::new();
    let mut odd_counter = 0;
    let mut even_counter = 1;
    counters.insert(0, (0, 1));
    let mut visited_even = HashSet::new();
    let mut visited_odd = HashSet::new();
    visited_even.insert(start);
    let mut queue = Vec::new();
    queue.push(start);
    let mut odd = true;
    let mut counter = 1;

    loop {
        let mut new_positions = Vec::new();
        while let Some(position) = queue.pop() {
            let e = (position.0, position.1 + 1);
            let s = (position.0 + 1, position.1);
            let w = (position.0, position.1 - 1);
            let n = (position.0 - 1, position.1);
            for new_position in [e, s, w, n] {
                if !garden.contains(&(modulo(new_position.0, max.0), modulo(new_position.1, max.1)))
                    && new_position.0 >= 0
                    && new_position.1 >= 0
                    && new_position.0 < max.0
                    && new_position.1 < max.1
                {
                    if odd {
                        if !visited_odd.contains(&new_position) {
                            visited_odd.insert(new_position);
                            new_positions.push(new_position);
                            odd_counter += 1;
                        }
                    } else {
                        if !visited_even.contains(&new_position) {
                            visited_even.insert(new_position);
                            new_positions.push(new_position);
                            even_counter += 1;
                        }
                    }
                }
            }
        }
        counters.insert(counter, (odd_counter, even_counter));
        queue = new_positions;
        if queue.len() == 0 {
            while counter < 263 {
                counters.insert(counter, (odd_counter, even_counter));
                counter += 1;
            }
            return counters;
        }
        odd = !odd;
        counter += 1;
    }
}

fn modulo(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn do_the_line(input: &str) -> (HashSet<(i64, i64)>, (i64, i64), (i64, i64)) {
    let mut row: i64 = 0;
    let mut col = 0;
    let mut garden = HashSet::new();
    let mut start = (0, 0);
    for line in input.lines() {
        col = 0;
        for c in line.chars() {
            if c == '#' {
                garden.insert((row, col));
            }
            if c == 'S' {
                start = (row, col);
            }
            col += 1;
        }
        row += 1;
    }
    (garden, start, (col, row))
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
        assert_eq!(part_1(&read_file("example.txt"), 6), 16);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), 64), 3642);
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt"), 5000), todo!());
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), 26501365), 608603023105276);
    }
}
