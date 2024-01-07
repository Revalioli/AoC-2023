use crate::*;
use regex::Regex;

run_day!{day1}

pub fn part1(input : &str) -> u32 {
    input.lines().fold(0,
        |sum, l| {
            let mut f = l.chars().filter(|c| c.is_ascii_digit());
            let first = f.next().unwrap().to_digit(10).unwrap();
            let last = f.last().unwrap_or_default()
                                .to_digit(10).unwrap_or(first);
            sum + first*10 + last
        }
    )
}

fn parse_number(num:&str) -> u32 {
    match num {
        "one"   | "1" => 1,
        "two"   | "2" => 2,
        "three" | "3" => 3,
        "four"  | "4" => 4,
        "five"  | "5" => 5,
        "six"   | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine"  | "9" => 9,
        other => panic!("Got {other} which is not a correct number")
    }
}

pub fn part2(input : &str) -> u32 {
    let mut sum = 0;
    let numbers = "one|two|three|four|five|six|seven|eight|nine|[1-9]";

    /* ".*" is greedy, it will try to match as much characters as possible.
     * That way, "({numbers}).*$" actually tries to match as much characters
     * as possible between the number and the end of the line, and this is
     * the case with the first occurence of ({number}).
     * Same the other way with "^.*({numbers})". 
     */
    let start = format!("({numbers}).*$");
    let end = format!("^.*({numbers})");

    let reg_start = Regex::new(&start).unwrap();
    let reg_end = Regex::new(&end).unwrap();

    for l in input.lines() {
        let (_, [first]) = reg_start.captures(l).unwrap().extract();
        let (_, [last] ) = reg_end.captures(l).unwrap().extract();
        sum += parse_number(first)*10 + parse_number(last);
    }

    sum
}
