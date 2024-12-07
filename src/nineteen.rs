use std::collections::HashSet;

use crate::{direction::Direction, point::Point};

type Wire = HashSet<Point>;

fn generate_wire_points(start: Point, length: i64, direction: Direction) -> Vec<Point> {
    (0..=length)
        .map(|i| direction.get_index(start.x, start.y, i).into())
        .collect()
}

pub fn three(data: String, _: i64) -> i64 {
    let wires: Vec<Wire> = data
        .split("\n")
        .map(|s| {
            let mut position: Point = (0, 0).into();

            s.split(",")
                .map(|s| {
                    let chars: Vec<char> = s.chars().collect();
                    let direction = match chars[0] {
                        'U' => Direction::Up,
                        'D' => Direction::Down,
                        'R' => Direction::Right,
                        'L' => Direction::Left,
                        _ => unimplemented!(),
                    };

                    let raw_length: String = chars[1..].iter().collect();
                    let length = raw_length.parse().unwrap();
                    let old_position = position;
                    position = direction.get_index(position.x, position.y, length).into();
                    generate_wire_points(old_position, length, direction)
                })
                .flatten()
                .collect()
        })
        .collect();

    wires[0]
        .intersection(&wires[1])
        .filter(|i| **i != Point { x: 0, y: 0 })
        .map(|i| i.x.abs() + i.y.abs())
        .min()
        .unwrap()
}

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
