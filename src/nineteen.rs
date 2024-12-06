pub fn one(data: String, part: i64) -> i64 {
    let fuelFunc = if part == 1 {
        calculate_fuel
    } else {
        |mass: i64| -> i64 { total_fuel(mass) - mass }
    };

    data.split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .map(fuelFunc)
        .sum()
}

fn total_fuel(mass: i64) -> i64 {
    if mass <= 0 {
        return 0;
    }

    mass + total_fuel(calculate_fuel(mass))
}

fn calculate_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}
