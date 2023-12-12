use core::panic;
use itertools::Itertools;
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
    let rows = do_the_line(input);
    let mut sum = 0;
    for row in rows {
        let pos = row.0.to_string();
        let mut grp = row.1;
        grp.reverse();
        let g = grp.pop().unwrap();
        let mut cashe: HashMap<(String, Vec<i64>, i64, i64), i64> = HashMap::new();
        let temp = recurse(pos, grp, g, 0, &mut cashe);
        sum += temp;
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let rows = do_the_line(input);
    let mut sum = 0;
    for row in rows {
        let (spr, mut grp) = unfold(row.0, row.1.clone());
        grp.reverse();
        let g = grp.pop().unwrap();
        let mut cashe: HashMap<(String, Vec<i64>, i64, i64), i64> = HashMap::new();
        sum += recurse(spr, grp, g, 0, &mut cashe);
    }
    sum
}

fn do_the_line(input: &str) -> Vec<(&str, Vec<i64>)> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let (springs, groups) = line.split(" ").collect_tuple().unwrap();
        let groups = groups
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        rows.push((springs, groups));
    }
    rows
}

fn unfold(springs: &str, groups: Vec<i64>) -> (String, Vec<i64>) {
    let mut new_groups = Vec::new();
    new_groups.append(&mut groups.clone());
    new_groups.append(&mut groups.clone());
    new_groups.append(&mut groups.clone());
    new_groups.append(&mut groups.clone());
    new_groups.append(&mut groups.clone());
    let new_springs = String::from(
        springs.to_owned() + "?" + springs + "?" + springs + "?" + springs + "?" + springs,
    );
    (new_springs, new_groups)
}

fn recurse(
    spr: String,
    mut groups: Vec<i64>,
    mut group: i64,
    mut count: i64,
    cashe: &mut HashMap<(String, Vec<i64>, i64, i64), i64>,
) -> i64 {
    if cashe.contains_key(&(spr.clone(), groups.clone(), group, count)) {
        return cashe[&(spr.clone(), groups.clone(), group, count)];
    }
    let mut springs = spr.chars().peekable();
    while let Some(s) = springs.next() {
        let groups_left = groups.len();
        let last = springs.peek() == None;
        if s == '?' {
            let mut sum = 0;
            // #, can never trigger a new group
            let (ok, _) = is_ok(count, group, groups_left as i64, last, '#');
            if ok {
                let springs: String = springs.clone().collect();
                let temp = recurse(springs.clone(), groups.clone(), group, count + 1, cashe);
                sum += temp;
                cashe.insert((springs.clone(), groups.clone(), group, count + 1), temp);
            }

            // .
            let (ok, new_group) = is_ok(count, group, groups_left as i64, last, '.');
            if ok {
                if new_group {
                    let g_temp = groups.pop();
                    if g_temp.is_some() {
                        group = g_temp.unwrap();
                    } else {
                        group = 0;
                    }
                    count = 0;
                }
                let springs: String = springs.clone().collect();
                let temp = recurse(springs.clone(), groups.clone(), group, count, cashe);
                sum += temp;
                cashe.insert((springs.clone(), groups.clone(), group, count), temp);
            }
            return sum;
        } else {
            let (ok, new_group) = is_ok(count, group, groups_left as i64, last, s);
            if ok {
                if s == '#' {
                    count += 1;
                }
                if new_group {
                    let g_temp = groups.pop();
                    if g_temp.is_some() {
                        group = g_temp.unwrap();
                    } else {
                        group = 0;
                    }
                    count = 0;
                }
            } else {
                return 0;
            }
        }
    }
    return 1;
}

fn is_ok(mut count: i64, group: i64, groups_left: i64, last: bool, s: char) -> (bool, bool) {
    //let mut is_ok = false;
    if last && groups_left > 0 {
        return (false, false);
    }
    if s == '#' {
        count += 1;
        if group == count {
            return (true, false);
        } else if group > count && !last {
            return (true, false);
        } else {
            return (false, false);
        }
    } else if s == '.' {
        if group == count {
            return (true, true);
        } else if last && group > 0 {
            return (false, false);
        } else if count == 0 {
            return (true, false);
        } else {
            return (false, false);
        }
    } else {
        panic!("Unknown char: {}", s);
    }
}

// For historical accuracy, this was the original part 1

/*fn count_correct(brutes: Vec<Vec<char>>, groups: Vec<i64>) -> i64 {
    let mut sum = 0;
    for brute in brutes {
        //println!("Brute: {}", brute.iter().collect::<String>());
        let mut brt = brute.iter().peekable();
        let mut counts = Vec::new();
        let mut counter = 0;
        while let Some(c) = brt.next() {
            if *c == '.' {
                if counter == 0 {
                    //ignore, no # found yet
                    continue;
                } else {
                    counts.push(counter);
                    counter = 0;
                }
            } else if *c == '#' {
                counter += 1;
                if brt.peek() == None {
                    counts.push(counter);
                }
            }
        }
        if groups == counts {
            sum += 1;
        }
    }
    //println!("sum: {}", sum);
    sum / 2
}*/

/*fn brute_the_force(spr: String) -> Vec<Vec<char>> {
    let mut new_s1 = Vec::new();
    let mut new_s2 = Vec::new();
    //let mut springs = springs.chars().collect_vec();
    //println!("springs: {}", spr);
    let mut result = Vec::new();
    let mut springs = spr.chars();
    let mut split = false;
    while let Some(s) = springs.next() {
        if s == '?' {
            split = true;
            let mut springs1: Vec<char> = springs.clone().collect();
            let mut springs2 = springs.clone().collect();
            new_s1.push('#');
            new_s2.push('.');
            new_s1.append(&mut springs1);
            new_s2.append(&mut springs2);
            result.append(&mut brute_the_force(
                new_s1.clone().into_iter().collect::<String>(),
            ));
            result.append(&mut brute_the_force(
                new_s2.clone().into_iter().collect::<String>(),
            ));
            break;
        } else {
            new_s1.push(s);
            new_s2.push(s);
        }
    }
    if !split {
        let mut res = Vec::new();
        res.push(new_s1);
        res.push(new_s2);
        result.append(&mut res);
    }
    result
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
        assert_eq!(part_1(&read_file("example.txt")), 21);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 7251);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 525152);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 2128386729962);
    }
}
