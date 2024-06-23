use std::{ops::Range, cmp::min};

use crate::*;

run_day!{day5}

/// Ranges are sorted
struct Converter {
    source_ranges: Vec<Range<u64>>,
    destination_ranges: Vec<Range<u64>>
}

impl Converter {
    /// Create a Converter from input strings.
    ///
    /// Conversion ranges are sorted, and identity conversion ranges are added, covering all possible u64 numbers.
    fn from_input(input: &str) -> Converter {
        let mut conv = Converter {
            source_ranges: Vec::new(),
            destination_ranges: Vec::new(),
        };

        let mut unsorted_ranges: Vec<(Range<u64>, Range<u64>)> = Vec::new();

        for l in input.lines().skip(1) {
            let mut temp = l.split_whitespace().map(|n| -> u64 { n.parse().unwrap() });
            iter2vars!(temp, [dst_start, src_start, len]);

            unsorted_ranges.push( (src_start..src_start+len, dst_start..dst_start+len) )
        }

        unsorted_ranges.sort_by(
            | (s1, _), (s2, _) | s1.start.partial_cmp(&s2.start).unwrap()
        );

        let (sorted_src_range, sorted_dst_range): (Vec<_>, Vec<_>) =
            unsorted_ranges.into_iter().unzip();

        if sorted_src_range[0].start > 0 {
            conv.source_ranges.push(0..sorted_src_range[0].start);
            conv.destination_ranges.push(0..sorted_src_range[0].start);
        }

        conv.source_ranges.push(sorted_src_range[0].clone());
        conv.destination_ranges.push(sorted_dst_range[0].clone());

        for i in 1..sorted_src_range.len() {
            if sorted_src_range[i].start > sorted_src_range[i-1].end {
                conv.source_ranges.push(sorted_src_range[i-1].end..sorted_src_range[i].start);
                conv.destination_ranges.push(sorted_src_range[i-1].end..sorted_src_range[i].start);
            }

            conv.source_ranges.push(sorted_src_range[i].clone());
            conv.destination_ranges.push(sorted_dst_range[i].clone());
        }

        conv.source_ranges.push(sorted_src_range.last().unwrap().end..u64::MAX);
        conv.destination_ranges.push(sorted_src_range.last().unwrap().end..u64::MAX);

        conv
    }

    fn convert(&self, val: &Seeds) -> Vec<Seeds> {
        match val {
            Seeds::SeedOne(seed) => {
                vec![Seeds::SeedOne(self.convert_single_seed(seed))]
            },
            Seeds::SeedRange(seeds) => {
                self.convert_range_seeds(seeds)
                    .into_iter().map(|s| Seeds::SeedRange(s))
                    .collect()
            }
        }
    }

    fn convert_single_seed(&self, seed: &u64) -> u64 {
        for (r, d) in self.source_ranges.iter().zip(self.destination_ranges.iter()) {
            if r.contains(&seed) {
                return d.start + (seed - r.start);
            }
        }
        *seed
    }

    fn convert_range_seeds(&self, seeds: &Range<u64>) -> Vec<Range<u64>> {
        let mut converted_seeds: Vec<Range<u64>> = Vec::new();

        for (r, d) in self.source_ranges.iter().zip(self.destination_ranges.iter()) {

            if seeds.end <= r.start || r.end <= seeds.start { continue; }
            if seeds.end > r.start {
                if seeds.start < r.start {
                    if seeds.end > r.end {
                        converted_seeds.push(d.start..d.end)
                    } else {
                        converted_seeds.push(d.start..(d.start + seeds.end - r.start))
                    }
                } else if seeds.end < r.end {
                    converted_seeds.push(
                        (d.start + seeds.start - r.start)..(d.start + seeds.end - r.start)
                    )
                } else if seeds.start < r.end {
                    converted_seeds.push(
                        (d.start + seeds.start - r.start)..d.end
                    )
                }
            }

        }

        converted_seeds
    }

}

enum Seeds {
    SeedOne(u64),
    SeedRange(Range<u64>)
}

struct Almanac {
    seeds: Vec<Seeds>,
    converters: Vec<Converter>
}

fn parse(input: &str, range_seeds: bool) -> Almanac {
    let mut blocks = input.split("\n\n");

    let parsed_seeds: Vec<u64> = blocks.next().unwrap()
                            .split_whitespace().skip(1)
                            .map(|n| n.parse().unwrap()).collect();

    let seeds: Vec<Seeds> =
        if range_seeds {
            parsed_seeds.chunks_exact(2)
            .map(|s| Seeds::SeedRange(s[0]..s[0]+s[1])).collect()
        } else {
            parsed_seeds.into_iter().map(|s| Seeds::SeedOne(s)).collect()
        };

    let mut converters: Vec<Converter> = Vec::new();

    for b in blocks {
        converters.push(Converter::from_input(b))
    }

    Almanac{ seeds, converters }
}

pub fn part1(input: &str) -> u64 {
    let a = parse(input, false);
    let mut res = u64::MAX;

    for mut seed in a.seeds {
        for c in &a.converters {
            seed = c.convert(&seed).pop().unwrap();
        }

        if let Seeds::SeedOne(val) = seed {
            res = min(res, val);
        } else {
            panic!("Found seed not of type SeedOne")
        }
    }

    res
}

pub fn part2(input: &str) -> u64 {
    let a = parse(input, true);
    let mut res = u64::MAX;

    for seeds in a.seeds {

        let mut seed_ranges = vec![seeds];

        for c in &a.converters {
            let mut new_ranges: Vec<Seeds> = Vec::new();
            for r in &seed_ranges {
                new_ranges.append(&mut c.convert(r));
            }

            seed_ranges = new_ranges;
        }

        for s in &seed_ranges {
            if let Seeds::SeedRange(r) = s {
                res = min(res, r.start);
            } else {
                panic!("Not seed range")
            }
        }

    }

    res
}