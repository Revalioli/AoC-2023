use std::{env, io, fs};

/* Common */
#[macro_export]
macro_rules! run_day {
    ( $d:ident ) => {
        pub fn run() -> io::Result<()> {
            println!("===[ {} ]===", std::stringify!($d));
            let file_name = format!( "inputs/input_{}", std::stringify!($d) );
            let input = read_data( &file_name )?;
            println!("Part 1 : {}", part1(&input) );
            println!("Part 2 : {}", part2(&input) );
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! iter2vars {
    ( $i:ident, [$($e:ident),+] ) => {
        $(
            let $e = $i.next().unwrap();
        )+
    }
}

macro_rules! day_run_vec {
    ( $( $d:ident ),* ) => { [ $( $d::run, )* ] };
}

fn read_data(file_name: &str) -> io::Result<String> {
    let input = fs::read_to_string(file_name)?;
    Ok(input.trim_end().to_string())
}

/* ==== Main ==== */

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let run_days =
        day_run_vec![
            day1, day2, day3, day4, day5
        ];

    let args : Vec<String> = env::args().collect();
    match args.get(1) {
        Some(d) => {
            // Run specific day
            let num : usize = d.parse()?;
            run_days[num-1]()?;
        },
        None => {
            // Run all days
            for day in run_days{
                day()?;
            }
        }
    }
    Ok(())
}


/* Test inputs */

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_day {
        (   $d:ident,
            $test_name_1:ident, $test_name_2:ident,
            $input_1:literal, $result_1:literal,
            $input_2:literal, $result_2:literal
        ) => {
            test_day!{$d, part1, $test_name_1, $input_1, $result_1}
            test_day!{$d, part2, $test_name_2, $input_2, $result_2}
        };
        (   $d:ident,
            $test_name_1:ident, $test_name_2:ident,
            $input:literal, $result_1:literal, $result_2:literal
        ) => {
            test_day!{ $d, $test_name_1, $test_name_2, $input, $result_1, $input, $result_2 }
        };
        ( $d:ident, $part:ident, $test_name:ident, $input:literal, $result:literal ) => {
            #[test]
            fn $test_name() { assert_eq!( $d::$part($input), $result); }
        };
    }

    test_day!{ day1, day1_test1, day1_test2,
        "1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet", 142,

        "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen", 281
    }

    test_day!{ day2, day2_test1, day2_test2,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        8, 2286
    }

    test_day!{ day3, day3_test1, day3_test2,
        "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..",
        4361, 467835
    }

    test_day!{ day4, day4_test1, day4_test2,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        13, 30
    }

    test_day!{ day5, day5_test1, day5_test2,
        "seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4",
        35, 46
    }

}