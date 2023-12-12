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

fn part_1(input: &str) -> i32 {
    let cards = do_the_line(input);
    let mut sum = 0;
    let base: i32 = 2;
    for (_, winning, have) in cards {
        let mut counter = 0;
        for win in winning {
            if have.contains(&win) {
                counter += 1;
            }
        }
        if counter > 0 {
            sum += base.pow(counter - 1);
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let cards = do_the_line(input);
    let mut queue: Vec<i32> = Vec::new();
    let mut sum = 0;
    for card in cards.iter() {
        sum += 1;
        queue.push(card.0);
    }
    while queue.len() > 0 {
        let card = queue.pop().unwrap();
        let (card, winning, have) = cards[card as usize - 1].clone();
        let mut counter = 0;
        for win in winning {
            if have.contains(&win) {
                counter += 1;
            }
        }
        while counter > 0 {
            queue.push(card + counter);
            sum += 1;
            counter -= 1;
        }
    }
    sum
}

fn do_the_line(input: &str) -> Vec<(i32, Vec<i32>, Vec<i32>)> {
    let re = regex::Regex::new(r": | \| ").unwrap();
    let re2 = regex::Regex::new(r" +").unwrap();
    let mut cards: Vec<(i32, Vec<i32>, Vec<i32>)> = Vec::new();
    for line in input.lines() {
        let trim_line = re2.replace_all(line, " ");
        let split_out: Vec<&str> = re.split(&trim_line).collect();
        let card: i32 = split_out[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let winning = split_out[1]
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let have = split_out[2]
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        cards.push((card, winning, have));
    }
    cards
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
        assert_eq!(part_1(&read_file("example.txt")), 13);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 22488);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 30);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 7013204);
    }
}
