use core::panic;
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
        part_2_opt(&read_file("example.txt"))
    );
    println!("Answer to Part 2: {}", part_2_opt(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let mut lowest = i64::MAX;
    let (seeds, sections) = do_the_line(input);
    for mut seed in seeds {
        for sec in sections.iter() {
            for row in sec.iter() {
                if seed >= row[1] && seed <= row[1] + row[2] {
                    seed += row[0] - row[1];
                    break;
                }
            }
        }
        if seed < lowest {
            lowest = seed;
        }
    }
    lowest
}

fn part_2(input: &str) -> i64 {
    // brute force, run with --release
    let mut lowest = i64::MAX;
    let (seeds, sections) = do_the_line(input);
    let mut _pairs = seeds.iter().peekable();
    while _pairs.len() > 0 {
        let pair = (_pairs.next().unwrap(), _pairs.next().unwrap());
        for mut seed in *pair.0..(pair.0 + pair.1 - 1) {
            for sec in sections.iter() {
                for row in sec.iter() {
                    if seed >= row[1] && seed <= row[1] + row[2] - 1 {
                        seed += row[0] - row[1];
                        break;
                    }
                }
            }
            if seed < lowest {
                lowest = seed;
            }
        }
    }
    lowest
}

fn part_2_opt(input: &str) -> i64 {
    //remade for fast runtime
    let mut lowest = i64::MAX;
    let (seeds, sections) = do_the_line(input);
    let mut _pairs = seeds.iter().peekable();
    let mut pairs = Vec::new();
    //create seed ranges, start, stop, and shifted flag to ensure we only shift once per section
    while _pairs.len() > 0 {
        let (start, range) = (_pairs.next().unwrap(), _pairs.next().unwrap());
        let seed = (*start, start + range - 1, false);
        pairs.push(seed);
    }
    let mut maps = Vec::new();
    for sec in sections.iter() {
        let mut convert_map = Vec::new();
        for row in sec.iter() {
            let distance = row[0] - row[1];
            let start = row[1];
            let end = row[1] + row[2] - 1;
            convert_map.push((start, end, distance));
        }
        maps.push(convert_map);
    }
    //To ensure alignment, flip execution and step through maps, then rows instead of starting with seeds
    for cmap in maps.iter() {
        for row in cmap.iter() {
            let mut new_pairs = Vec::new();
            //iterate through all seeds, if they overlap the row, split and shift the overlapping part, mark shifted
            for seeds in pairs.iter_mut() {
                if seeds.2 {
                    //see if already shifted, if so pass along
                    new_pairs.push(seeds.clone());
                } else if seeds.1 < row.0 || seeds.0 > row.1 {
                    //no overlap, do nothing, push seed to next row
                    new_pairs.push(seeds.clone());
                } else if seeds.0 >= row.0 && seeds.1 <= row.1 {
                    //full overlap, shift all
                    seeds.0 += row.2;
                    seeds.1 += row.2;
                    seeds.2 = true;
                    new_pairs.push(seeds.clone());
                } else if seeds.0 <= row.0 && seeds.1 <= row.1 {
                    //partial overlap low, split and shift overlapping
                    let seeds1 = (seeds.0, row.0 - 1, false);
                    let seeds2 = (row.0 + row.2, seeds.1 + row.2, true);
                    new_pairs.push(seeds1);
                    new_pairs.push(seeds2);
                } else if seeds.0 >= row.0 && seeds.1 >= row.1 {
                    //partial overlap high, split and shift overlapping
                    let seeds1 = (seeds.0 + row.2, row.1 + row.2, true);
                    let seeds2 = (row.1 + 1, seeds.1, false);
                    new_pairs.push(seeds1);
                    new_pairs.push(seeds2);
                } else if seeds.0 < row.0 && seeds.1 > row.1 {
                    //full overlap low high, split and shift overlapping
                    let seeds1 = (seeds.0, row.0 - 1, false);
                    let seeds2 = (row.0 + row.2, row.1 + row.2, true);
                    let seeds3 = (row.1 + 1, seeds.1, false);
                    new_pairs.push(seeds1);
                    new_pairs.push(seeds2);
                    new_pairs.push(seeds3);
                } else {
                    println!("row {} {} {}", row.0, row.1, row.2);
                    println!("seeds {} {} {}", seeds.0, seeds.1, seeds.2);
                    panic!("Shouldn't be here");
                }
            }
            pairs = new_pairs.clone();
        }
        for seeds in pairs.iter_mut() {
            //reset shifted flag before next section
            seeds.2 = false;
        }
    }
    for seeds in pairs.iter() {
        if seeds.0 < lowest {
            lowest = seeds.0;
        }
    }
    lowest
}

fn do_the_line(input: &str) -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let _sections = input.split("\n\n").collect_vec();
    let _seeds = _sections[0].split(' ').collect_vec()[1..].to_vec();
    let mut sections = Vec::new();
    //dest range start,  source range start,  range length

    let mut seeds = Vec::new();
    for s in _seeds {
        //println!("seed: {}", s);
        seeds.push(s.parse::<i64>().unwrap());
    }
    for sec in _sections[1..].iter() {
        let mut section = Vec::new();
        for line in sec.lines().skip(1) {
            let nums = line
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec();
            section.push(nums);
        }
        sections.push(section);
    }

    (seeds, sections)
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
        assert_eq!(part_1(&read_file("example.txt")), 35);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 462648396);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2_opt(&read_file("example.txt")), 46);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2_opt(&read_file("input.txt")), 2520479);
    }
}
