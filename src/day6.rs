use crate::*;

run_day!{day6}

struct Race {
    time: u32,
    distance: u32,
}

pub fn part1(input: &str) -> usize {
    let mut l = input.lines();
    let times = l.next().unwrap().split_whitespace()
        .skip(1).map(|t| t.parse().unwrap());
    let distances = l.next().unwrap().split_whitespace()
        .skip(1).map(|d| d.parse().unwrap());

    let races: Vec<Race> = times.zip(distances).map(|(t, d)| Race { time: t, distance: d }).collect();
    let mut res = 1;

    for r in &races {
        res *= (0..r.time).map(|i| i*(r.time-i)).filter(|d| d > &r.distance).count();
    }

    res
}

pub fn part2(input: &str) -> u64 {
    let mut l = input.lines();
    let time: u64 = l.next().unwrap().split_whitespace()
        .skip(1).collect::<Vec<&str>>().join("").parse().unwrap();
    let distance: u64 = l.next().unwrap().split_whitespace()
        .skip(1).collect::<Vec<&str>>().join("").parse().unwrap();

    let delta = time*time-4*distance;
    let sqrt_delta = (delta as f64).sqrt();

    let x1 = ((time as f64) - sqrt_delta) / 2.0;
    let x2 = ((time as f64) + sqrt_delta) / 2.0;

    (x2.ceil() as u64) - (x1.ceil() as u64)
}