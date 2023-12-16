use core::panic;
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
    let start_beam = ((0, 0), (0, 1));
    let mut energized = HashMap::new();
    let contraption = do_the_line(input);
    let max = (
        contraption.keys().map(|x| x.0).max().unwrap(),
        contraption.keys().map(|x| x.1).max().unwrap(),
    );
    let mut sum = 0;
    println!("Max: {:?}", max);
    beam_it(start_beam, &mut energized, &contraption, &max);
    for _ in energized {
        sum += 1;
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let contraption = do_the_line(input);
    let max = (
        contraption.keys().map(|x| x.0).max().unwrap(),
        contraption.keys().map(|x| x.1).max().unwrap(),
    );

    let mut max_beam = 0;
    for n in 0..max.0 {
        let mut energized = HashMap::new();
        let start_beam = ((n, 0), (0, 1));
        let mut sum = 0;
        beam_it(start_beam, &mut energized, &contraption, &max);
        for _ in energized {
            sum += 1;
        }
        if sum > max_beam {
            max_beam = sum;
        }
    }
    for n in 0..max.0 {
        let mut energized = HashMap::new();
        let start_beam = ((n, max.1), (0, -1));
        let mut sum = 0;
        beam_it(start_beam, &mut energized, &contraption, &max);
        for _ in energized {
            sum += 1;
        }
        if sum > max_beam {
            max_beam = sum;
        }
    }
    for n in 0..max.1 {
        let mut energized = HashMap::new();
        let start_beam = ((0, n), (1, 0));
        let mut sum = 0;
        beam_it(start_beam, &mut energized, &contraption, &max);
        for _ in energized {
            sum += 1;
        }
        if sum > max_beam {
            max_beam = sum;
        }
    }
    for n in 0..max.1 {
        let mut energized = HashMap::new();
        let start_beam = ((max.0, n), (-1, 0));
        let mut sum = 0;
        beam_it(start_beam, &mut energized, &contraption, &max);
        for _ in energized {
            sum += 1;
        }
        if sum > max_beam {
            max_beam = sum;
        }
    }
    max_beam
}

fn beam_it(
    mut beam: ((i64, i64), (i64, i64)),
    energized: &mut HashMap<(i64, i64), (bool, bool, bool, bool)>,
    contraption: &HashMap<(i64, i64), char>,
    max: &(i64, i64),
) {
    //while we are on the board
    while beam.0 .0 >= 0 && beam.0 .1 >= 0 && beam.0 .0 < max.0 + 1 && beam.0 .1 < max.1 + 1 {
        //if we are not energized, energize and remember direction. Board countains loops
        if !energized.contains_key(&beam.0) {
            let mut state = (false, false, false, false);
            match beam.1 {
                (0, 1) => {
                    state.0 = true;
                }
                (1, 0) => {
                    state.1 = true;
                }
                (0, -1) => {
                    state.2 = true;
                }
                (-1, 0) => {
                    state.3 = true;
                }
                _ => {
                    panic!("Beam error");
                }
            }
            energized.insert(beam.0, state);
        //See if weve passed in this direction before, if so exit, otherwise mark it as visited
        } else {
            let mut state = energized.get(&beam.0).unwrap().clone();
            match beam.1 {
                (0, 1) => {
                    if state.0 {
                        break;
                    } else {
                        state.0 = true
                    };
                }
                (1, 0) => {
                    if state.1 {
                        break;
                    } else {
                        state.1 = true
                    };
                }
                (0, -1) => {
                    if state.2 {
                        break;
                    } else {
                        state.2 = true
                    };
                }
                (-1, 0) => {
                    if state.3 {
                        break;
                    } else {
                        state.3 = true
                    };
                }
                _ => {
                    panic!("Beam error");
                }
            }
            energized.insert(beam.0, state);
        }
        //see if we hit a mirror
        if contraption.contains_key(&beam.0) {
            //change direction
            match contraption.get(&beam.0).unwrap() {
                '/' => match beam.1 {
                    (0, 1) => {
                        beam.1 = (-1, 0);
                    }
                    (1, 0) => {
                        beam.1 = (0, -1);
                    }
                    (0, -1) => {
                        beam.1 = (1, 0);
                    }
                    (-1, 0) => {
                        beam.1 = (0, 1);
                    }
                    _ => {
                        panic!("Mirror error 1");
                    }
                },
                '\\' => match beam.1 {
                    (0, 1) => {
                        beam.1 = (1, 0);
                    }
                    (1, 0) => {
                        beam.1 = (0, 1);
                    }
                    (0, -1) => {
                        beam.1 = (-1, 0);
                    }
                    (-1, 0) => {
                        beam.1 = (0, -1);
                    }
                    _ => {
                        panic!("Mirror error 2");
                    }
                },
                //or split
                '|' => match beam.1 {
                    (1, 0) | (-1, 0) => {
                        //nothing
                    }
                    (0, 1) | (0, -1) => {
                        //println!("Splitting1");
                        //println!("Beam: {:?}", beam);
                        beam.1 = (1, 0);
                        //split out one direction

                        beam_it(
                            ((beam.0 .0 - 1, beam.0 .1), (-1, 0)),
                            energized,
                            contraption,
                            max,
                        );
                    }
                    _ => {
                        panic!("Mirror erro 3");
                    }
                },
                '-' => match beam.1 {
                    (1, 0) | (-1, 0) => {
                        //println!("Splitting-");
                        //println!("Beam: {:?}", beam);
                        beam.1 = (0, 1);
                        //split out one direction
                        beam_it(
                            ((beam.0 .0, beam.0 .1 - 1), (0, -1)),
                            energized,
                            contraption,
                            max,
                        );
                    }
                    (0, 1) | (0, -1) => {
                        //nothing
                    }
                    _ => {
                        panic!("Mirror error 4");
                    }
                },
                _ => {
                    panic!("Unknown contraption part");
                }
            }
        }
        //move, for split this is already done at recursion
        beam.0 = (beam.0 .0 + beam.1 .0, beam.0 .1 + beam.1 .1);
    }
}

fn do_the_line(input: &str) -> HashMap<(i64, i64), char> {
    let mut row: i64 = 0;
    let mut col;
    let mut contraption = HashMap::new();
    for line in input.lines() {
        col = 0;
        for c in line.chars() {
            if c != '.' {
                contraption.insert((row, col), c);
            }
            col += 1;
        }
        row += 1;
    }

    contraption
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
        assert_eq!(part_1(&read_file("example.txt")), 46);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 7788);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 51);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 7987);
    }
}
