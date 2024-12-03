use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

pub fn two(data: &Path, part: i64) -> i64 {
    fn safe_p1(report: Vec<i64>) -> bool {
        if report[0] == report[1] {
            return false;
        }

        let increasing = report[0] < report[1];
        for pair in report.windows(2) {
            if increasing {
                if pair[0] >= pair[1] || pair[1] - 3 > pair[0] {
                    return false;
                }
            } else {
                if pair[0] <= pair[1] || pair[0] - 3 > pair[1] {
                    return false;
                }
            }
        }
        true
    }
    fn safe_p2(report: Vec<i64>) -> bool {
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if safe_p1(report) {
                return true;
            }
        }

        false
    }

    let safe = if part == 1 { safe_p1 } else { safe_p2 };

    fs::read_to_string(data)
        .expect("should be able to read data")
        .split("\n")
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(safe)
        .fold(0, |acc, b| if b { acc + 1 } else { acc })
}

pub fn one(data: &Path, part: i64) -> i64 {
    let mut locations: (Vec<i64>, Vec<i64>) = fs::read_to_string(data)
        .expect("should be able to read data")
        .split('\n')
        .map(|l| {
            let parts: Vec<&str> = l.split("   ").collect();
            (
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            )
        })
        .unzip();

    if part == 1 {
        locations.0.sort();
        locations.1.sort();

        locations
            .0
            .iter()
            .zip(locations.1.iter())
            .map(|(l, r)| (l - r).abs())
            .sum()
    } else {
        let mut count = HashMap::<i64, i64>::new();
        for location in locations.1 {
            match count.get(&location) {
                Some(v) => count.insert(location, v + 1),
                None => count.insert(location, 1),
            };
        }

        locations
            .0
            .iter()
            .map(|l| match count.get(l) {
                Some(v) => v * l,
                None => 0,
            })
            .sum()
    }
}
