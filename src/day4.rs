use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

// Common

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

fn parse_card(input: &str) -> Card {
    let (card, nums) = input.split_once(":").unwrap();
    let id: u32 = card.split_whitespace().last().unwrap().parse().unwrap();
    let (winning, yours) = nums.split_once("|").unwrap();

    return Card {
        id,
        winning_numbers: winning
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        your_numbers: yours
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
    };
}

fn parse_input() -> Vec<Card> {
    let contents = fs::read_to_string("./inputs/day4.txt").expect("No file found");
    let lines = contents.split("\n");

    return lines.filter(|l| l.len() > 0).map(parse_card).collect();
}

fn num_winning_cards(card: &Card) -> u32 {
    let mut winning_cards = 0;
    for your_num in card.your_numbers.iter() {
        if card.winning_numbers.contains(your_num) {
            winning_cards += 1;
        }
    }
    return winning_cards;
}

// Part 1

fn card_score(card: &Card) -> u32 {
    let winning_cards = num_winning_cards(card);
    if winning_cards > 0 {
        return u32::pow(2, winning_cards - 1);
    }
    return 0;
}

fn part1() -> u32 {
    let cards = parse_input();
    return cards.iter().map(card_score).sum();
}

// Part 2

fn num_copies(cards: &Vec<Card>) -> u32 {
    let mut card_copies: HashMap<u32, u32> = HashMap::new();
    for card in cards {
        let this_card_copies = 1 + *card_copies.entry(card.id).or_insert(0);
        let winning_cards = num_winning_cards(&card);
        for delta in 0..winning_cards {
            let copies = card_copies.entry(card.id + delta + 1).or_insert(0);
            *copies += this_card_copies;
        }
    }
    return card_copies.into_values().sum();
}

fn part2() -> u32 {
    let cards = parse_input();
    return cards.len() as u32 + num_copies(&cards);
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
        assert_eq!(part1(), 22674);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 5747443);
    }
}
