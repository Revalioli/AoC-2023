use crate::*;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;

run_day!{day7}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand: [char; 5],
    bid: u64,
    kind: HandKind,
    use_jokers: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct FormatError;

impl HandKind {
    fn from_chars(chars: &[char; 5], with_joker: bool) -> HandKind {
        // Count how many of each character
        let mut card_counts: HashMap<char, u8> = HashMap::new();
        let mut joker_count = 0;

        for c in chars {
            match card_counts.get_mut(c) {
                Some(n) => *n += 1,
                None => { card_counts.insert(*c, 1); }
            }
        }

        if with_joker {
            joker_count = card_counts.remove(&'J').unwrap_or_default();
        }
        // Sort by number of occurences
        let mut counts: Vec<u8> = card_counts.into_iter().map(|x| x.1).collect();
        counts.sort();

        if with_joker {
            if counts.is_empty() {
                counts.push(0);
            }
            *counts.last_mut().unwrap() += joker_count;
        }

        match counts[..] {
            [ 1, 1, 1, 1, 1 ] => Self::HighCard,
            [ 1, 1, 1, 2 ] => Self::Pair,
            [ 1, 2, 2 ] => Self::TwoPair,
            [ 1, 1, 3 ] => Self::ThreeOfAKind,
            [ 2, 3 ] => Self::FullHouse,
            [ 1, 4 ] => Self::FourOfAKind,
            [ 5 ] => Self::FiveOfAKind,
            _ => panic!("Impossible values {:?}", counts),
        }
    }

}

impl Hand {
    fn from_str(s: &str, with_joker: bool) -> Result<Self, FormatError> {
        let mut it: std::str::SplitWhitespace = s.split_whitespace();

        let hand = it.next().ok_or(FormatError)?
            .chars().collect::<Vec<char>>()
            .try_into().or(Err(FormatError))?;
        let kind = HandKind::from_chars(&hand, with_joker); 

        Ok(
            Hand {
                hand,
                bid: it.next().ok_or(FormatError)?.parse().or(Err(FormatError))?,
                kind,
                use_jokers: with_joker,
            }
        )
    }
}

impl FromStr for HandKind {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: [char; 5] = s.chars().collect::<Vec<char>>()
        .try_into().or(Err(FormatError))?;

        Ok(HandKind::from_chars(&chars, false))
    }

}

impl FromStr for Hand {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hand::from_str(s, false)
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.kind.cmp(&other.kind);
        let val_func = if self.use_jokers { card2val_with_jokers } else { card2val };
        if ord == Ordering::Equal {
            let self_values: Vec<u8> = self.hand.iter().map(val_func).collect();
            let other_values: Vec<u8> = other.hand.iter().map(val_func).collect();

            self_values.cmp(&other_values)
        } else {
            ord
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn parse_input(input: &str) -> Result<Vec<Hand>, FormatError> {
    let mut r = Vec::new();
    for l in input.lines() {
        r.push(l.parse()?);
    }
    Ok(r)
}

fn card2val(c: &char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0
    }
}

fn card2val_with_jokers(c: &char) -> u8 {
    match c {
        'J' => 1,
        _ => card2val(c)
    }
}

fn compute_score(mut hands: Vec<Hand>) -> u64 {
    hands.sort();
    hands.iter().enumerate().fold(0, 
        |acc, (i, h)|
            h.bid * (i as u64 + 1) + acc
    )
}


pub fn part1(input: &str) -> u64 {
    let hands = match parse_input(input) {
        Ok(h) => h,
        Err(_) => return 0,
    };
    compute_score(hands)
}

pub fn part2(input: &str) -> u64 {
    let hands: Vec<Hand> = input.lines()
        .map(|h| Hand::from_str(h, true).unwrap())
        .collect();
    compute_score(hands)
}