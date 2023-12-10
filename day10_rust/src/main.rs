use core::panic;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example2.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!(
        "Answer to Part 2 test 1: {}",
        part_2(&read_file("example3.txt"))
    );
    println!(
        "Answer to Part 2 test 2: {}",
        part_2(&read_file("example4.txt"))
    );
    println!(
        "Answer to Part 2 test 3: {}",
        part_2(&read_file("example5.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i32 {
    let (start, matrix) = do_the_line(input);
    find_direction(start, matrix).0
}

fn part_2(input: &str) -> i32 {
    let (start, matrix) = do_the_line(input);
    find_direction(start, matrix).1
}

fn find_direction(start: (i32, i32), matrix: HashMap<(i32, i32), char>) -> (i32, i32) {
    if start.0 > 0 && ['|', '7', 'F'].contains(&matrix[&(start.0 - 1, start.1)]) {
        return do_the_loop(start, (start.0 - 1, start.1), matrix, 3);
    }
    if ['-', '7', 'J'].contains(&matrix[&(start.0, start.1 + 1)]) {
        return do_the_loop(start, (start.0, start.1 + 1), matrix, 0);
    }
    if ['|', 'L', 'J'].contains(&matrix[&(start.0 + 1, start.1)]) {
        return do_the_loop(start, (start.0 + 1, start.1), matrix, 1);
    }
    if start.1 > 0 && ['-', 'L', 'F'].contains(&matrix[&(start.0, start.1 - 1)]) {
        return do_the_loop(start, (start.0, start.1 - 1), matrix, 2);
    }
    panic!("No direction found");
}

fn do_the_loop(
    start: (i32, i32),
    mut pos: (i32, i32),
    matrix: HashMap<(i32, i32), char>,
    mut direction: i32,
) -> (i32, i32) {
    let mut counter = 1;
    let mut inner_parts_right = HashSet::new();
    let mut inner_parts_left = HashSet::new();
    let mut loop_parts = HashSet::new();
    loop_parts.insert(start);
    let mut max = (0, 0);
    loop {
        let c = matrix[&pos];
        loop_parts.insert(pos);
        if max.0 < pos.0 {
            max.0 = pos.0;
        }
        if max.1 < pos.1 {
            max.1 = pos.1;
        }
        if c == 'S' {
            break;
        }
        match c {
            '|' => match direction {
                3 => {
                    inner_parts_left.insert((pos.0, pos.1 - 1));
                    inner_parts_right.insert((pos.0, pos.1 + 1));
                    pos.0 -= 1;
                    counter += 1;
                }
                1 => {
                    inner_parts_left.insert((pos.0, pos.1 + 1));
                    inner_parts_right.insert((pos.0, pos.1 - 1));
                    pos.0 += 1;
                    counter += 1;
                }
                _ => panic!("Invalid direction"),
            },
            '-' => match direction {
                0 => {
                    inner_parts_left.insert((pos.0 - 1, pos.1));
                    inner_parts_right.insert((pos.0 + 1, pos.1));
                    pos.1 += 1;
                    counter += 1;
                }
                2 => {
                    inner_parts_left.insert((pos.0 + 1, pos.1));
                    inner_parts_right.insert((pos.0 - 1, pos.1));
                    pos.1 -= 1;
                    counter += 1;
                }
                _ => panic!("Invalid direction"),
            },
            '7' => match direction {
                0 => {
                    inner_parts_left.insert((pos.0 - 1, pos.1));
                    inner_parts_left.insert((pos.0, pos.1 + 1));
                    direction = 1;
                    pos.0 += 1;
                    counter += 1;
                }
                3 => {
                    inner_parts_right.insert((pos.0 - 1, pos.1));
                    inner_parts_right.insert((pos.0, pos.1 + 1));
                    direction = 2;
                    pos.1 -= 1;
                    counter += 1;
                }
                _ => panic!("Invalid direction"),
            },
            'F' => match direction {
                3 => {
                    inner_parts_left.insert((pos.0 - 1, pos.1));
                    inner_parts_left.insert((pos.0, pos.1 - 1));
                    direction = 0;
                    pos.1 += 1;
                    counter += 1;
                }
                2 => {
                    inner_parts_right.insert((pos.0 - 1, pos.1));
                    inner_parts_right.insert((pos.0, pos.1 - 1));
                    direction = 1;
                    pos.0 += 1;
                    counter += 1;
                }
                _ => panic!("Invalid direction"),
            },
            'J' => match direction {
                1 => {
                    inner_parts_left.insert((pos.0 + 1, pos.1));
                    inner_parts_left.insert((pos.0, pos.1 + 1));
                    direction = 2;
                    pos.1 -= 1;
                    counter += 1;
                }
                0 => {
                    inner_parts_right.insert((pos.0 + 1, pos.1));
                    inner_parts_right.insert((pos.0, pos.1 + 1));
                    direction = 3;
                    pos.0 -= 1;
                    counter += 1;
                }
                _ => panic!("Invalid direction"),
            },
            'L' => match direction {
                1 => {
                    inner_parts_right.insert((pos.0 + 1, pos.1));
                    inner_parts_right.insert((pos.0, pos.1 - 1));
                    direction = 0;
                    pos.1 += 1;
                    counter += 1;
                }
                2 => {
                    inner_parts_left.insert((pos.0 + 1, pos.1));
                    inner_parts_left.insert((pos.0, pos.1 - 1));
                    direction = 3;
                    pos.0 -= 1;
                    counter += 1;
                }
                _ => panic!("Invalid direction"),
            },

            _ => panic!("Invalid character"),
        }
    }
    max.0 += 2;
    max.1 += 2;
    let insides_left = test_inside(loop_parts.clone(), inner_parts_left, max);
    let insides_right = test_inside(loop_parts.clone(), inner_parts_right, max);

    (counter / 2, insides_left.max(insides_right))
}

fn print_loop(loop_parts: HashSet<(i32, i32)>, max: (i32, i32)) {
    for i in 0..(max.0) {
        for j in 0..(max.1) {
            if loop_parts.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!(".");
        println!();
    }
    println!(".");
}

fn test_inside(
    loop_parts: HashSet<(i32, i32)>,
    mut inner_parts: HashSet<(i32, i32)>,
    max: (i32, i32),
) -> i32 {
    //will have odd number of loop part to edge, always look north because lazy
    //change to get large print as well
    let x = 0;
    if max.0 < x {
        //change to get large print as well
        println!("Loop parts");
        print_loop(loop_parts.clone(), max);
        println!("Before prune");
        print_loop(inner_parts.clone(), max);
    }
    //remove all loop parts from inner parts
    for n in inner_parts.clone() {
        if loop_parts.contains(&n) {
            inner_parts.remove(&n);
        }
    }

    if max.0 < x {
        println!("After prune");
        print_loop(inner_parts.clone(), max);
    }
    let mut full_inner = HashSet::new();
    for p in inner_parts {
        //check all directions and add any boxes we find untill we hit the loop or the edge, this fills large inner holes
        for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let mut n = p;
            loop {
                if n.0 > max.0 || n.1 > max.1 || n.0 < 0 || n.1 < 0 {
                    //found edge, we are on the outside not inside, discard entire set and return 0
                    return 0;
                }
                if loop_parts.contains(&(n)) {
                    //found loop wall
                    break;
                }
                //still inside, add to set and move
                full_inner.insert(n);
                n.0 += d.0;
                n.1 += d.1;
            }
        }
    }
    full_inner.len() as i32
}

fn do_the_line(input: &str) -> ((i32, i32), HashMap<(i32, i32), char>) {
    let mut matrix: HashMap<(i32, i32), char> = HashMap::new();
    let mut row = 0;
    let mut start = (-1, -1);

    for line in input.lines() {
        let mut column = 0;
        for c in line.chars() {
            matrix.insert((row, column), c);
            if c == 'S' {
                start = (row, column);
            }
            column += 1;
        }
        row += 1;
    }

    (start, matrix)
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
        assert_eq!(part_1(&read_file("example2.txt")), 8);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 6860);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example4.txt")), 8);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 343);
    }
}
