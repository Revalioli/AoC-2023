use std::{ops::Range, cmp::min};

use crate::*;

run_day!{day5}

struct Converter {
    source_ranges: Vec<Range<u64>>,
    destination_ranges: Vec<Range<u64>>
}

impl Converter {
    fn from_input(input: &str) -> Converter {
        let mut conv = Converter {
            source_ranges: Vec::new(),
            destination_ranges: Vec::new(),
        };
        
        for l in input.lines().skip(1) {
            let mut temp = l.split_whitespace().map(|n| -> u64 { n.parse().unwrap() });
            iter2vars!(temp, [dst_start, src_start, len]);
            conv.destination_ranges.push(dst_start..dst_start+len);
            conv.source_ranges.push(src_start..src_start+len);
        }

        conv
    }

    fn convert(&self, val: u64) -> u64 {
        for i in 0..self.source_ranges.len() {
            let r = &self.source_ranges[i]; 

            if r.contains(&val) {
                return self.destination_ranges[i].start + (val - r.start);
            }
        }
        val
    }
}

struct Almanac {
    seeds: Vec<u64>,
    converters: Vec<Converter>
}

fn parse(input: &str) -> Almanac {
    let mut blocks = input.split("\n\n");
    let seeds: Vec<u64> = blocks.next().unwrap()
                            .split_whitespace().skip(1)
                            .map(|n| n.parse().unwrap()).collect();
    let mut converters: Vec<Converter> = Vec::new();

    for b in blocks {
        converters.push(Converter::from_input(b))
    }

    Almanac{ seeds, converters }
}

pub fn part1(input: &str) -> u64 {
    let a = parse(input);
    let mut res = u64::MAX;

    for mut val in a.seeds {
        for c in &a.converters {
            val = c.convert(val);
        }
        res = min(res, val);
    }

    res
}

pub fn part2(input: &str) -> u64 {
    let a = parse(input);
    let mut res = u64::MAX;

    for s in a.seeds.chunks_exact(2) {

        for mut val in s[0]..s[0]+s[1] {
            for c in &a.converters {
                val = c.convert(val);
            }
            res = min(res, val);
        }

    }
    
    res
}