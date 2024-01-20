use crate::*;

run_day!{day4}

struct Card {
    quantity: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

fn parse(input: &str) -> Vec<Card> {

    input.lines().map(
        |l| {
            let mut temp = l.split('|');
            let (left, right) = (
                temp.next().unwrap().split_whitespace(),
                temp.next().unwrap().split_whitespace()
            );
            
            Card {
                quantity: 1,
                winning_numbers: left.skip(2).map(|n| n.parse().unwrap()).collect(),
                numbers: right.map(|n| n.parse().unwrap()).collect()
            }
        }
    ).collect()

}

pub fn part1(input: &str) -> u32 {
    let cards = parse(input);

    cards.iter().fold(0,
        |acc, c| {
            let winning_nums = c.numbers.iter().filter(
                |n| c.winning_numbers.contains(n)
            ).count() as u32;

            if winning_nums > 0 {
                acc + 2u32.pow(winning_nums-1)
            }
            else { acc }
        }
    )
}

pub fn part2(input: &str) -> u32 {
    let mut cards = parse(input);
    
    for i in 0..cards.len() {
        let c = &cards[i];
        let quant = c.quantity;

        let winning_nums = c.numbers.iter().filter(
            |n| c.winning_numbers.contains(n)
        ).count();

        for j in i+1..i+1+winning_nums {
            cards[j].quantity += quant;
        }
    }

    cards.iter().map(|c| c.quantity).sum()
}