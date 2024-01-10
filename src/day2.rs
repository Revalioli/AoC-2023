use core::num;

use crate::*;

run_day!{day2}

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for l in input.lines() {
        let mut split = l.split_whitespace()
                            .map(|x| x.trim_end_matches([',', ';', ':']));
        let id : u32 = split.nth(1).unwrap()
                            .parse().unwrap();

        let numbers = split.clone().step_by(2).map(|n| n.parse::<u32>());
        let colours = split.skip(1).step_by(2);

        let mut possible = true;

        for (n, c) in numbers.zip(colours) {
            let n = n.unwrap(); 
            match c {
                "red" => if n > 12 { possible = false; break; }
                "green" => if n > 13 { possible = false; break; }
                "blue" => if n > 14 { possible = false; break; }
                _ => { panic!("Uknown colour {c}") }
            }
        }

        if possible { sum += id }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    input.lines().fold(0,
        |acc, l| {
            let mut split = l.split_whitespace()
                    .map(|x| x.trim_end_matches([',', ';', ':']));
            let id : u32 = split.nth(1).unwrap()
                    .parse().unwrap();

            let numbers = split.clone().step_by(2).map(|n| n.parse::<u32>());
            let colours = split.skip(1).step_by(2);

            let mut maxs = (0u32, 0u32, 0u32); // red, green, blue

            for (n, c) in numbers.zip(colours) {
                let n = n.unwrap();
                match c {
                    "red" => if n > maxs.0 { maxs.0 = n }
                    "green" => if n > maxs.1 { maxs.1 = n }
                    "blue" => if n > maxs.2 { maxs.2 = n }
                    _ => { panic!("Uknown colour {c}") }
                }
            }

            acc + maxs.0 * maxs.1 * maxs    .2
        }
    )
}