use std::collections::HashMap;

use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1::<10>(&read_file("example.txt"))
    );
    println!(
        "Answer to Part 1: {}",
        part_1::<100>(&read_file("input.txt"))
    );
    println!(
        "Answer to Part 2 test: {}",
        part_2::<10>(&read_file("example.txt"))
    );
    println!(
        "Answer to Part 2: {}",
        part_2::<100>(&read_file("input.txt"))
    );
}

fn part_1<const N: usize>(input: &str) -> i64 {
    let mut rocks: [[i8; N]; N] = do_the_line(input);
    tilt(&mut rocks);
    let mut sum = 0;
    for r in 0..N {
        for c in 0..N {
            if rocks[r][c] == 1 {
                sum += N as i64 - r as i64;
            }
        }
    }

    sum
}

fn part_2<const N: usize>(input: &str) -> i64 {
    let mut rocks: [[i8; N]; N] = do_the_line(input);
    let mut cache = HashMap::new();
    let mut first_hit = ([[0; N]; N], 0);
    let mut n = 1;
    let mut endgame = false;
    while n < 1000000001 {
        if !endgame {
            if cache.contains_key(&rocks) {
                if first_hit.1 == 0 {
                    first_hit = (rocks, n);
                } else if first_hit.0 == rocks {
                    endgame = true;
                    n = 1000000000 - (1000000000 - n) % (n - first_hit.1);
                }
            } else {
                cache.insert(rocks, n);
            }
        }
        tilt_cycle(&mut rocks);
        n += 1;
    }
    let mut sum = 0;

    for r in 0..N {
        for c in 0..N {
            if rocks[r][c] == 1 {
                sum += N as i64 - r as i64;
            }
        }
    }

    sum
}

fn tilt<const N: usize>(rocks: &mut [[i8; N]; N]) {
    for r in 0..N {
        for c in 0..N {
            if rocks[r][c] == 1 {
                let mut y = r;
                while y > 0 && rocks[y - 1][c] == 0 {
                    y -= 1;
                }
                rocks[r][c] = 0;
                rocks[y][c] = 1;
            }
        }
    }
}

fn tilt_cycle<const N: usize>(rocks: &mut [[i8; N]; N]) {
    //NORTH
    for r in 0..N {
        for c in 0..N {
            if rocks[r][c] == 1 {
                let mut y = r;
                while y > 0 && rocks[y - 1][c] == 0 {
                    y -= 1;
                }
                rocks[r][c] = 0;
                rocks[y][c] = 1;
            }
        }
    }
    //WEST
    for c in 0..N {
        for r in 0..N {
            if rocks[r][c] == 1 {
                let mut x = c;
                while x > 0 && rocks[r][x - 1] == 0 {
                    x -= 1;
                }
                //println!("{} {}", r, c);
                rocks[r][c] = 0;
                rocks[r][x] = 1;
            }
        }
    }
    //SOUTH
    for r in (0..N).rev() {
        for c in 0..N {
            if rocks[r][c] == 1 {
                let mut y = r;
                while y < N - 1 && rocks[y + 1][c] == 0 {
                    y += 1;
                }
                rocks[r][c] = 0;
                rocks[y][c] = 1;
            }
        }
    }
    //EAST
    for c in (0..N).rev() {
        for r in 0..N {
            if rocks[r][c] == 1 {
                let mut x = c;
                while x < N - 1 && rocks[r][x + 1] == 0 {
                    x += 1;
                }
                //println!("{} {}", r, c);
                rocks[r][c] = 0;
                rocks[r][x] = 1;
            }
        }
    }
}

fn do_the_line<const N: usize>(input: &str) -> [[i8; N]; N] {
    let mut row = 0;
    let mut col;
    let mut rock_rows = [[0; N]; N];
    for line in input.lines() {
        col = 0;
        for c in line.chars() {
            if c == '#' {
                rock_rows[row][col] = 2;
            } else if c == 'O' {
                rock_rows[row][col] = 1;
            }
            col += 1;
        }
        row += 1;
    }
    rock_rows
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
        assert_eq!(part_1::<10>(&read_file("example.txt")), 136);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1::<100>(&read_file("input.txt")), 110565);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2::<10>(&read_file("example.txt")), 64);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2::<100>(&read_file("input.txt")), 89845);
    }
}
