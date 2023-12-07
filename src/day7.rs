use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// Common

#[derive(Debug)]
struct CardHand {
    hand: String,
    poker_type: String,
    bid: u32,
}

fn file_reader() -> io::BufReader<File> {
    let file = File::open("./inputs/day7.txt").unwrap();
    return io::BufReader::new(file);
}

fn hand_ranking(hand: &str) -> u32 {
    match hand {
        "Five of a kind" => 7,
        "Four of a kind" => 6,
        "Full house" => 5,
        "Three of a kind" => 4,
        "Two pair" => 3,
        "One pair" => 2,
        "High card" => 1,
        _ => panic!("Unknown poker hand {}", hand),
    }
}

fn parse_line(
    reader: &mut io::BufReader<File>,
    poker_type_fn: &dyn Fn(&str) -> String,
) -> Result<CardHand, &'static str> {
    let mut buffer = String::new();
    let len = reader.read_line(&mut buffer).expect("No file found");

    if len == 0 {
        return Err("EOF");
    }

    let (hand, bid) = buffer.split_once(" ").unwrap();
    return Ok(CardHand {
        hand: hand.to_string(),
        poker_type: poker_type_fn(&hand),
        bid: bid.trim().parse().unwrap(),
    });
}

fn get_sorted_hands(
    poker_type_fn: &dyn Fn(&str) -> String,
    card_ranking_fn: &dyn Fn(&char) -> u32,
) -> Vec<CardHand> {
    let mut reader = file_reader();
    let mut hands: Vec<CardHand> = Vec::new();

    loop {
        let hand = parse_line(&mut reader, poker_type_fn);
        if hand.is_err() {
            break;
        }
        hands.push(hand.unwrap());
    }

    hands.sort_by(|a, b| {
        let a_rank = hand_ranking(&a.poker_type);
        let b_rank = hand_ranking(&b.poker_type);

        if a_rank == b_rank {
            let mut a_chars = a.hand.chars();
            let mut b_chars = b.hand.chars();

            for _i in 0..5 {
                let card_a = a_chars.next().unwrap();
                let card_b = b_chars.next().unwrap();

                if card_a != card_b {
                    return card_ranking_fn(&card_a).cmp(&card_ranking_fn(&card_b));
                }
            }
        }

        return a_rank.cmp(&b_rank);
    });

    return hands;
}

fn get_winnings(
    poker_type_fn: &dyn Fn(&str) -> String,
    card_ranking_fn: &dyn Fn(&char) -> u32,
) -> u32 {
    let mut sum = 0;

    for (idx, hand) in get_sorted_hands(poker_type_fn, card_ranking_fn)
        .iter()
        .enumerate()
    {
        sum += hand.bid * (idx as u32 + 1);
    }

    return sum;
}

// Part 1

fn poker_type_1(hand: &str) -> String {
    let mut h: HashMap<char, u32> = HashMap::new();
    for char in hand.chars() {
        *h.entry(char).or_insert(0) += 1;
    }

    if h.len() == 2 && h.values().any(|v| *v == 3) {
        return "Full house".to_string();
    }

    if h.len() == 3 && !h.values().any(|v| *v == 3) {
        return "Two pair".to_string();
    }

    for count in h.values() {
        if *count == 5 {
            return "Five of a kind".to_string();
        };
        if *count == 4 {
            return "Four of a kind".to_string();
        };
        if *count == 3 {
            return "Three of a kind".to_string();
        };
        if *count == 2 {
            return "One pair".to_string();
        };
    }

    return "High card".to_string();
}

fn card_ranking_1(card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn part1() -> u32 {
    return get_winnings(&poker_type_1, &card_ranking_1);
}

// Part 2

fn poker_type_2(hand: &str) -> String {
    let mut h: HashMap<char, u32> = HashMap::new();
    for char in hand.chars() {
        *h.entry(char).or_insert(0) += 1;
    }

    if h.contains_key(&'J') {
        if h.len() == 2 {
            return "Five of a kind".to_string();
        }

        if h.len() == 3 {
            if h[&'J'] == 3 || h[&'J'] == 2 || h.values().any(|v| *v == 3) {
                return "Four of a kind".to_string();
            }
            return "Full house".to_string();
        }

        if h.len() == 4 {
            return "Three of a kind".to_string();
        }

        if h.len() == 5 {
            return "One pair".to_string();
        }
    }

    if h.len() == 2 && h.values().any(|v| *v == 3) {
        return "Full house".to_string();
    }

    return poker_type_1(&hand);
}

fn card_ranking_2(card: &char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn part2() -> u32 {
    return get_winnings(&poker_type_2, &card_ranking_2);
}

// Main

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 250946742);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 251824095);
    }
}
