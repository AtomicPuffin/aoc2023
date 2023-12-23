use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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
    let trails = do_the_line(input);
    hike(&trails)
}

fn part_2(input: &str) -> i64 {
    let trails = do_the_line(input);
    hike2(&trails)
}

fn hike(trails: &HashMap<(i64, i64), char>) -> i64 {
    let mut hike = HashSet::new();

    let max = (
        trails.keys().map(|x| x.0).max().unwrap(),
        trails.keys().map(|x| x.1).max().unwrap(),
    );

    hike.insert((0, 1));
    let mut heap = VecDeque::new();
    heap.push_back(((0, 1), hike));

    let mut completed = Vec::new();

    while let Some(current) = heap.pop_front() {
        for direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let mut next_pos = (current.0 .0 + direction.0, current.0 .1 + direction.1);
            if next_pos == (max.0, max.1 - 1) {
                completed.push(current.1.clone());
            }
            if current.1.contains(&next_pos) {
                continue;
            }
            let mut next_curr;
            if trails.contains_key(&next_pos) {
                match trails.get(&next_pos).unwrap() {
                    '#' => {
                        continue;
                    }
                    '<' => {
                        if direction == (0, 1) {
                            continue;
                        }
                        next_curr = current.clone();
                        next_curr.1.insert(next_pos);
                        next_pos.1 -= 1;
                    }
                    '>' => {
                        if direction == (0, -1) {
                            continue;
                        }
                        next_curr = current.clone();
                        next_curr.1.insert(next_pos);
                        next_pos.1 += 1;
                    }
                    '^' => {
                        if direction == (1, 0) {
                            continue;
                        }
                        next_curr = current.clone();
                        next_curr.1.insert(next_pos);
                        next_pos.0 -= 1;
                    }
                    'v' => {
                        if direction == (-1, 0) {
                            continue;
                        }
                        next_curr = current.clone();
                        next_curr.1.insert(next_pos);
                        next_pos.0 += 1;
                    }
                    _ => {
                        panic!("Unknown character in trails");
                    }
                }
            } else {
                next_curr = current.clone();
            }
            next_curr.1.insert(next_pos);
            heap.push_back((next_pos, next_curr.1.clone()));
        }
    }
    completed.sort_by(|a, b| b.len().cmp(&a.len()));

    if false {
        for y in 0..max.0 + 1 {
            for x in 0..max.1 + 1 {
                if trails.contains_key(&(y, x)) {
                    print!("{}", trails.get(&(y, x)).unwrap());
                } else if completed[0].contains(&(y, x)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    completed[0].len() as i64 - 1
}

fn hike2(trails: &HashMap<(i64, i64), char>) -> i64 {
    let size = (
        trails.keys().map(|x| x.0).max().unwrap(),
        trails.keys().map(|x| x.1).max().unwrap(),
    );
    let end = (size.0, size.1 - 1);
    let trails_new = build_tree(trails, end);
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    visited.insert((0, 1));
    queue.push(((0, 1), 0, visited));
    let mut max: i64 = 0;

    while let Some(state) = queue.pop() {
        for (next_node, dist) in trails_new.get(&state.0).unwrap() {
            if state.2.contains(next_node) {
                continue;
            }
            if next_node == &end {
                if dist + state.1 > max {
                    max = dist + state.1;
                }
            }
            let mut next_state = state.clone();
            next_state.0 = *next_node;
            next_state.1 += dist;
            next_state.2.insert(*next_node);

            queue.push(next_state);
        }
    }
    max
}
fn build_tree(
    trails: &HashMap<(i64, i64), char>,
    end: (i64, i64),
) -> HashMap<(i64, i64), HashMap<(i64, i64), i64>> {
    let mut trails_new = HashMap::new();
    let mut queue = Vec::new();
    queue.push((0, 1));

    while let Some(node) = queue.pop() {
        if !trails_new.contains_key(&node) {
            trails_new.insert(node, HashMap::new());
        } else {
            continue;
        }
        for direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next_pos = (node.0 + direction.0, node.1 + direction.1);
            if trails.contains_key(&next_pos) {
                if trails.get(&next_pos).unwrap() == &'#' {
                    continue;
                }
            }
            let (next_node, dist) = traverse_path(trails, (next_pos, direction), end);
            trails_new.get_mut(&node).unwrap().insert(next_node, dist);
            queue.push(next_node);
        }
    }

    trails_new
}

fn traverse_path(
    trails: &HashMap<(i64, i64), char>,
    start: ((i64, i64), (i64, i64)),
    end: (i64, i64),
) -> ((i64, i64), i64) {
    let mut neighs = Vec::new();
    neighs.push(start);
    let mut current = ((0, 0), (0, 0));
    let mut distance = 0;
    while neighs.len() == 1 {
        distance += 1;
        current = neighs.pop().unwrap();
        for direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next_pos = (current.0 .0 + direction.0, current.0 .1 + direction.1);
            if next_pos == end {
                return (end, distance);
            }
            if direction == (current.1 .0 * -1, current.1 .1 * -1) {
                continue;
            }
            if next_pos == (0, 1) {
                return ((0, 1), distance);
            }

            if trails.contains_key(&next_pos) {
                if trails.get(&next_pos).unwrap() == &'#' {
                    continue;
                }
            }
            neighs.push((next_pos, direction));
        }
    }
    if neighs.len() == 0 {
        panic!("No path found {:?}", current.0);
    }
    (current.0, distance)
}
fn do_the_line(input: &str) -> HashMap<(i64, i64), char> {
    let mut row: i64 = 0;
    let mut col = 0;
    let mut trails = HashMap::new();
    for line in input.lines() {
        col = 0;
        for c in line.chars() {
            if c != '.' {
                trails.insert((row, col), c);
            }
            col += 1;
        }
        row += 1;
    }
    trails.insert((-1, 1), '#');
    trails.insert((row, col - 2), '#');
    trails
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
        assert_eq!(part_1(&read_file("example.txt")), 94);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 2414);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 154);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 6598);
    }
}
