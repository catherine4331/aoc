use std::{collections::HashMap, fs, path::Path};

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
