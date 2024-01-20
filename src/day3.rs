use std::ops::Range;
use std::cmp::{max, min};

use crate::*;

run_day!{day3}

struct Number {
    pos : (usize, Range<usize>), /* Line, range */
    val : u32
}

impl Number {
    fn in_symbol_area(&self, area: &(Range<usize>, Range<usize>)) -> bool {
        area.0.contains(&self.pos.0) && intersect(&area.1, &self.pos.1)
    }
}

fn get_symbol_area( s: &(usize, usize), nb_lines: usize, nb_cols: usize ) -> (Range<usize>, Range<usize>) {
    ( 
        max(s.0-1, 0)..min(s.0+2, nb_lines),
        max(s.1-1, 0)..min(s.1+2,  nb_cols)
    )
}

fn intersect<T>(r1: &Range<T>, r2: &Range<T>) -> bool
where
    T: Ord
{
    !(r2.end <= r1.start || r2.start >= r1.end)
}

fn parse(lines: Vec<&str>) -> (Vec<(usize, usize)>, Vec<Number>) {
    let mut buff = String::new();
    let mut symbols : Vec<(usize, usize)> = Vec::new(); // (Line, column)
    let mut numbers : Vec<Number> = Vec::new();

    let (nb_lines, nb_cols) = (lines.len(), lines[0].len());

    for (i, l) in lines.iter().enumerate() {
        /* Case were line ended with a number */
        if !buff.is_empty() {
            numbers.push(
                Number {
                    pos: ( i-1, nb_cols-buff.len()..nb_cols ),
                    val: buff.parse().unwrap()
                }
            );
            buff.clear();
        }

        for (j, c) in l.chars().enumerate() {

            if c.is_ascii_digit() {
                buff.push(c);
            } else {
            
                if !buff.is_empty() {
                    numbers.push(
                        Number {
                            pos: ( i, j-buff.len()..j ),
                            val: buff.parse().unwrap()
                        }
                    );
                    buff.clear();
                }

                if c != '.' {   /* Symbols */
                    symbols.push((i,j));
                }
            }

        }

    }
    /* Case were last line ended with a number */
    if !buff.is_empty() {
        numbers.push(
            Number {
                pos: ( nb_lines-1, nb_cols-buff.len()..nb_cols ),
                val: buff.parse().unwrap()
            }
        );
        buff.clear();
    }

    (symbols, numbers)
}


pub fn part1(input: &str) -> u32 {
    let lines : Vec<&str> = input.lines().collect();
    let (nb_lines, nb_cols) = (lines.len(), lines[0].len());
    let (symbols, numbers) = parse(lines);

    let mut sum: u32 = 0;

    for n in numbers {

        for s in &symbols {
            let sym_area = get_symbol_area(s, nb_lines, nb_cols);
            if n.in_symbol_area(&sym_area)
            {
                sum += n.val;
                break;
            }
        }
        
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let lines : Vec<&str> = input.lines().collect();
    let (nb_lines, nb_cols) = (lines.len(), lines[0].len());
    let (symbols, numbers) = parse(lines);

    let mut sum = 0;

    for s in symbols {
        let sym_area = get_symbol_area(&s, nb_lines, nb_cols);
        let mut adjacent_parts = 0u8;
        let mut ratio = 0;

        for n in &numbers {
            if n.in_symbol_area(&sym_area)
            {
                match adjacent_parts {
                    0 => ratio += n.val,
                    1 => ratio *= n.val,
                    _ => break
                }
                adjacent_parts += 1;
            }
        }

        if adjacent_parts == 2 { sum += ratio; }
    }

    sum
}