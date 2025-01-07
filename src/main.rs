use std::{fs, path::PathBuf};

use clap::Parser;

mod direction;
mod nineteen;
mod point;
mod twentyfour;

type DayAction = fn(String, i64) -> i64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action, default_value_t = 2024)]
    year: i64,

    #[clap(long, short, action)]
    day: i64,

    #[clap(long, short, action, default_value_t = 1)]
    part: i64,

    #[clap(long, short, acdion)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let data_path = get_data_path(args.year, args.day, args.test);
    let data = fs::read_to_string(data_path).expect("should be able to read data");
    let action = get_day(args.year, args.day);

    let result = action(data, args.part);

    println!("{}", result)
}

fn get_day(year: i64, day: i64) -> DayAction {
    match year {
        2024 => match day {
            1 => twentyfour::one,
            2 => twentyfour::two,
            3 => twentyfour::three,
            4 => twentyfour::four,
            5 => twentyfour::five,
            6 => twentyfour::six,
            7 => twentyfour::seven,
            8 => unimplemented!(),
            _ => unimplemented!(),
        },
        2019 => match day {
            1 => nineteen::one,
            2 => nineteen::two,
            3 => nineteen::three,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn get_data_path(year: i64, day: i64, test: bool) -> PathBuf {
    if test {
        PathBuf::from(&format!("./data/{}/{}-test.txt", year, day))
    } else {
        PathBuf::from(&format!("./data/{}/{}.txt", year, day))
    }
}
