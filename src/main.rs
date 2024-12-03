use std::path::{Path, PathBuf};

use clap::Parser;

mod twentyfive;

type DayAction = fn(&Path, i64) -> i64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action, default_value_t = 2024)]
    year: i64,

    #[clap(long, short, action)]
    day: i64,

    #[clap(long, short, action, default_value_t = 1)]
    part: i64,

    #[clap(long, short, action)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    let data_path = get_data_path(args.year, args.day, args.test);
    let action = get_day(args.year, args.day);

    let result = action(data_path.as_path(), args.part);

    println!("{}", result)
}

fn get_day(year: i64, day: i64) -> DayAction {
    match year {
        2025 => match day {
            1 => twentyfive::one,
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
