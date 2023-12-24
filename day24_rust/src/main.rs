use itertools::Itertools;

use std::fs;

use z3::ast::{Ast, Int};

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"), 7 as f64, 27 as f64)
    );
    println!(
        "Answer to Part 1: {}",
        part_1(
            &read_file("input.txt"),
            200000000000000 as i64 as f64,
            400000000000000 as i64 as f64
        )
    );
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str, min: f64, max: f64) -> i64 {
    let mut hails = do_the_line(input);
    let mut counter = 0;
    while let Some(hail) = hails.pop() {
        for hail2 in hails.clone() {
            if intersection_xy(hail, hail2, min, max) {
                counter += 1;
            }
        }
    }

    counter
}

fn part_2(input: &str) -> i64 {
    let hails = do_the_line(input);
    // select only 5 lines to get execution down, still correct
    z3_cheater(&hails[0..5])
}

fn z3_cheater(lines: &[((f64, f64, f64), (f64, f64, f64))]) -> i64 {
    // realized this was above my paygrade, so I drew heavy inspiration from the interwebs
    // I uderstand what it does at least...
    let context = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&context);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|n| Int::new_const(&context, n));

    let zero = Int::from_i64(&context, 0);
    for (i, &((x, y, z), (dx, dy, dz))) in lines.iter().enumerate() {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|n| Int::from_i64(&context, n as _));
        let t = Int::new_const(&context, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).unwrap();
    res.as_i64().unwrap()
}

fn intersection_xy(
    line_a: ((f64, f64, f64), (f64, f64, f64)),
    line_b: ((f64, f64, f64), (f64, f64, f64)),
    min: f64,
    max: f64,
) -> bool {
    let a = line_a.1 .1 / line_a.1 .0;
    let b = line_b.1 .1 / line_b.1 .0;
    let c = line_a.0 .1 - (line_a.1 .1 / line_a.1 .0) * line_a.0 .0;
    let d = line_b.0 .1 - (line_b.1 .1 / line_b.1 .0) * line_b.0 .0;

    if a == b {
        return false;
    }
    let x = (d - c) / (a - b);
    let y = a * (d - c) / (a - b) + c;

    if x < min as f64 || x > max as f64 {
        return false;
    }
    if y < min as f64 || y > max as f64 {
        return false;
    }
    if {
        test_past(line_a.0 .0 as f64, x, line_a.1 .0)
            || test_past(line_b.0 .0 as f64, x, line_b.1 .0)
            || test_past(line_a.0 .1 as f64, y, line_a.1 .1)
            || test_past(line_b.0 .1 as f64, y, line_b.1 .1)
    } {
        return false;
    }

    true
}

fn test_past(x_start: f64, x_intersect: f64, dx: f64) -> bool {
    if dx <= 0 as f64 {
        if x_intersect < x_start {
            return false;
        }
    } else {
        if x_intersect > x_start {
            return false;
        }
    }
    true
}

fn do_the_line(input: &str) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    let mut hails = Vec::new();
    let re = regex::Regex::new(r" +").unwrap();
    let trim_line = re.replace_all(input, " ");
    for line in trim_line.lines() {
        let (h, v) = line.split(" @ ").collect_tuple().unwrap();
        let (px, py, pz) = h
            .split(", ")
            .collect_vec()
            .iter()
            .map(|x| x.parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (vx, vy, vz) = v
            .split(", ")
            .collect_vec()
            .iter()
            .map(|x| x.parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        hails.push(((px, py, pz), (vx, vy, vz)));
    }

    hails
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
        assert_eq!(part_1(&read_file("example.txt"), 7 as f64, 27 as f64), 2);
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            part_1(
                &read_file("input.txt"),
                200000000000000 as i64 as f64,
                400000000000000 as i64 as f64
            ),
            18098
        );
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 47);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 886858737029295);
    }
}
