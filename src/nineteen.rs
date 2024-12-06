struct IntCodeRunner {
    data: Vec<usize>,
    position: usize,
}

impl IntCodeRunner {
    fn new(data: Vec<usize>) -> Self {
        Self { data, position: 0 }
    }

    fn next(&mut self) -> Option<()> {
        if self.position >= self.data.len() {
            return None;
        } else {
            match self.data[self.position] {
                1 => {
                    let new_position = self.data[self.position + 3];
                    let left = self.data[self.data[self.position + 1]];
                    let right = self.data[self.data[self.position + 2]];
                    self.data[new_position] = left + right;
                }
                2 => {
                    let new_position = self.data[self.position + 3];
                    let left = self.data[self.data[self.position + 1]];
                    let right = self.data[self.data[self.position + 2]];
                    self.data[new_position] = left * right;
                }
                99 => return None,
                _ => panic!("Invalid opcode"),
            }
        }

        self.position += 4;
        Some(())
    }
}

pub fn two(data: String, part: i64) -> i64 {
    let mut runner = IntCodeRunner::new(data.split(",").map(|s| s.parse().unwrap()).collect());

    if part == 1 {
        runner.data[1] = 12;
        runner.data[2] = 2;

        while runner.next() != None {}

        runner.data[0] as i64
    } else {
        for i in 0..100 {
            for j in 0..100 {
                let mut runner =
                    IntCodeRunner::new(data.split(",").map(|s| s.parse().unwrap()).collect());
                runner.data[1] = i;
                runner.data[2] = j;

                while runner.next() != None {}

                if runner.data[0] == 19690720 {
                    return (100 * i + j) as i64;
                }
            }
        }

        0
    }
}

pub fn one(data: String, part: i64) -> i64 {
    let fuel_func = if part == 1 {
        calculate_fuel
    } else {
        |mass: i64| -> i64 { total_fuel(mass) - mass }
    };

    data.split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .map(fuel_func)
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
