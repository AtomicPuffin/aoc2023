use core::panic;
use itertools::Itertools;
use std::collections::HashMap;

use std::collections::VecDeque;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example2.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let mut modules = do_the_line(input);
    let mut counter = 1000;

    let mut low = 0;
    let mut high = 0;
    while counter > 0 {
        let mut queue = VecDeque::new();
        queue.push_back((0, "broadcaster".to_string(), "button".to_string()));
        while let Some(current) = queue.pop_front() {
            if !modules.contains_key(&current.1) {
                continue;
            }
            match modules.get(&current.1).unwrap().0 {
                0 => {
                    // broadcaster
                    low += 1; // button low
                    for target in &modules.get(&current.1).unwrap().3 {
                        low += 1;
                        queue.push_back((0, target.to_string(), current.1.to_string()));
                    }
                }
                1 => {
                    // flip-flop
                    if current.0 == 1 {
                        //high pulse, ignore
                        continue;
                    } else {
                        // low pulse
                        if modules.get(&current.1).unwrap().1 == 1 {
                            // on, turn off, send low
                            modules.get_mut(&current.1).unwrap().1 = 0;
                            for target in &modules.get(&current.1).unwrap().3 {
                                low += 1;
                                queue.push_back((0, target.to_string(), current.1.to_string()));
                            }
                        } else {
                            // on, turn on,  send high
                            modules.get_mut(&current.1).unwrap().1 = 1;
                            for target in &modules.get(&current.1).unwrap().3 {
                                high += 1;
                                queue.push_back((1, target.to_string(), current.1.to_string()));
                            }
                        }
                    }
                }
                2 => {
                    // conjunction
                    // update memory
                    modules
                        .get_mut(&current.1)
                        .unwrap()
                        .2
                        .insert(current.2, current.0);
                    let mut all_high = true;
                    for (_, v) in &modules.get(&current.1).unwrap().2 {
                        if *v == 0 {
                            all_high = false;
                            break;
                        }
                    }
                    if all_high {
                        for target in &modules.get(&current.1).unwrap().3 {
                            low += 1;
                            queue.push_back((0, target.to_string(), current.1.to_string()));
                        }
                    } else {
                        for target in &modules.get(&current.1).unwrap().3 {
                            high += 1;
                            queue.push_back((1, target.to_string(), current.1.to_string()));
                        }
                    }
                }
                _ => {
                    panic!("Unknown operator runtime")
                }
            }
        }
        counter -= 1;
    }

    low * high
}

fn part_2(input: &str) -> i64 {
    let mut modules = do_the_line(input);
    let mut counter = 0;
    let mut series = HashMap::new();
    for s in [
        "qz".to_string(),
        "sk".to_string(),
        "sv".to_string(),
        "dr".to_string(),
    ] {
        series.insert(s, Vec::new());
    }
    loop {
        if counter == 10000 {
            // this is a wee bit lazy, I made the function to print whenever the registers send a 0, thinking there would be a repeating series
            // and then just looked at the output and noticed it was simply a repeating interval, and well below 10000.
            break;
        }
        let mut queue = VecDeque::new();
        queue.push_back((0, "broadcaster".to_string(), "button".to_string()));
        while let Some(current) = queue.pop_front() {
            if !modules.contains_key(&current.1) {
                continue;
            }
            match modules.get(&current.1).unwrap().0 {
                0 => {
                    // broadcaster
                    for target in &modules.get(&current.1).unwrap().3 {
                        queue.push_back((0, target.to_string(), current.1.to_string()));
                    }
                }
                1 => {
                    // flip-flop
                    if current.0 == 1 {
                        //high pulse, ignore
                        continue;
                    } else {
                        // low pulse
                        if modules.get(&current.1).unwrap().1 == 1 {
                            // on, turn off, send low
                            modules.get_mut(&current.1).unwrap().1 = 0;
                            for target in &modules.get(&current.1).unwrap().3 {
                                queue.push_back((0, target.to_string(), current.1.to_string()));
                            }
                        } else {
                            // on, turn on,  send high
                            modules.get_mut(&current.1).unwrap().1 = 1;
                            for target in &modules.get(&current.1).unwrap().3 {
                                queue.push_back((1, target.to_string(), current.1.to_string()));
                            }
                        }
                    }
                }
                2 => {
                    // conjunction
                    // update memory
                    modules
                        .get_mut(&current.1)
                        .unwrap()
                        .2
                        .insert(current.2, current.0);
                    let mut all_high = true;
                    for (_, v) in &modules.get(&current.1).unwrap().2 {
                        if *v == 0 {
                            all_high = false;
                            break;
                        }
                    }
                    if all_high {
                        if [
                            "qz".to_string(),
                            "sk".to_string(),
                            "sv".to_string(),
                            "dr".to_string(),
                        ]
                        .contains(&current.1)
                        {
                            series.get_mut(&current.1).unwrap().push(counter);
                        }
                        for target in &modules.get(&current.1).unwrap().3 {
                            queue.push_back((0, target.to_string(), current.1.to_string()));
                        }
                    } else {
                        for target in &modules.get(&current.1).unwrap().3 {
                            queue.push_back((1, target.to_string(), current.1.to_string()));
                        }
                    }
                }
                _ => {
                    panic!("Unknown operator runtime")
                }
            }
        }
        counter += 1;
    }
    let mut patterns = Vec::new();
    for (_, v) in series {
        let mut curr = 0;
        let mut pattern = 0;
        for i in v {
            pattern = i - curr;
            curr = i;
        }
        patterns.push(pattern);
    }
    let mut a = patterns.pop().unwrap();
    while let Some(b) = patterns.pop() {
        a = lcm(a, b);
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
fn gcd(a: i64, b: i64) -> i64 {
    let mut n = a.max(b);
    let mut d = a.min(b);
    while d != 0 {
        let rem = n % d;
        n = d;
        d = rem;
    }
    n
}

fn do_the_line(input: &str) -> HashMap<String, (i64, i64, HashMap<String, i64>, Vec<String>)> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (modu, targs) = line.split(" -> ").collect_tuple().unwrap();
        let m_type;
        let name;

        match &modu[0..1] {
            "b" => {
                m_type = 0;
                name = modu.to_string();
            }
            "%" => {
                m_type = 1;
                name = modu[1..].to_string();
            }
            "&" => {
                m_type = 2;
                name = modu[1..].to_string();
            }
            _ => {
                panic!("Unknown operator parse")
            }
        }

        let targets = targs.split(", ").map(|x| x.to_string()).collect_vec();
        modules.insert(name.clone(), (m_type, 0, HashMap::new(), targets));
    }
    let mut counters = HashMap::new();
    for (m, v) in modules.clone() {
        for t in v.3 {
            if counters.contains_key(&t) {
                counters.insert(t.clone(), counters.get(&t).unwrap() + 1);
            } else {
                counters.insert(t.clone(), 1);
            }
            if modules.contains_key(&t) {
                modules.get_mut(&t).unwrap().2.insert(m.clone(), 0);
            }
        }
    }
    modules
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
        assert_eq!(part_1(&read_file("example2.txt")), 11687500);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 898557000);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 238420328103151);
    }
}
