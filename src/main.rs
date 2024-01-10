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


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let run_days =
        day_run_vec![
            day1, day2
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
}