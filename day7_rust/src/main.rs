use core::cmp::Ordering;
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
        part_2(&read_file("example.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i32 {
    let mut hands = do_the_line(input, false);
    hands.sort_by(|a, b| b.cmp(a));
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i as i32 + 1);
        /*println!(
            "cards {} bid  {} type {} rank {}",
            hand.cards,
            hand.bid,
            hand.hand_type,
            i as i32 + 1
        );*/
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let mut hands = do_the_line(input, true);
    hands.sort_by(|a, b| b.cmp(a));
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid * (i as i32 + 1);
        /*println!(
            "cards {} bid  {} type {} rank {}",
            hand.cards,
            hand.bid,
            hand.hand_type,
            i as i32 + 1
        );*/
    }
    sum
}

#[derive(Clone, Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: String,
    bid: i32,
    hand_type: i32,
    jokers: bool,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (i, c) in self.cards.chars().enumerate() {
                if c == other.cards.chars().nth(i).unwrap() {
                    continue;
                } else {
                    let slf = numerate_card(c, self.jokers);
                    let otr = numerate_card(other.cards.chars().nth(i).unwrap(), self.jokers);
                    return otr.cmp(&slf);
                }
            }
        } else {
            return self.hand_type.cmp(&other.hand_type);
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

fn numerate_card(card: char, jokers: bool) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if jokers {
                1
            } else {
                11
            }
        }

        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    }
}

fn get_type(cards: &str, jokers: bool) -> i32 {
    let mut chars = cards.chars().collect_vec();
    chars.sort_by(|a, b| b.cmp(a));
    //println!("chars {:?}", chars);
    let mut count = 0;
    let mut counts = Vec::new();
    let mut char = ' ';
    let mut joker_count = 0;
    for c in chars {
        if c == 'J' && jokers {
            joker_count += 1;
            continue;
        }
        if count == 0 {
            char = c;
            count = 1;
        } else if c == char {
            count += 1;
        } else {
            counts.push(count);
            count = 1;
            char = c;
        }
    }
    counts.push(count);
    //println!("counts {:?}", counts);
    if joker_count == 5 {
        return 1; // 5 jokers
    }
    counts.sort_by(|a, b| b.cmp(a));
    counts[0] += joker_count;
    if counts[0] == 5 {
        return 1; //5 of a kind
    } else if counts.len() == 2 && counts[0] == 4 {
        return 2; //4 of a kind
    } else if counts.len() == 2 {
        return 3; // full house
    } else if counts.len() == 3 && counts[0] == 3 {
        return 4; // 2 pair
    } else if counts.len() == 3 {
        return 5; // 3 of a kind
    } else if counts.len() == 4 {
        return 6; // 1 pair
    } else {
        return 7; // high card
    }
}

fn do_the_line(input: &str, jokers: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split(' ').collect_tuple().unwrap();
        let hand_type = get_type(hand, jokers);
        hands.push(Hand {
            cards: hand.to_string(),
            bid: bid.parse::<i32>().unwrap(),
            hand_type: hand_type,
            jokers: jokers,
        });
    }
    hands
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
        assert_eq!(part_1(&read_file("example.txt")), 6440);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 249726565);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 5905);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 251135960);
    }
}
