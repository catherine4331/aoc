use std::{
    cmp,
    collections::{HashMap, HashSet},
    ops::Index,
};

use crate::direction::Direction;

enum Op {
    Add,
    Mult,
    Concat,
}

fn check_calibration(target: i64, inputs: Vec<i64>) -> bool {
    (0..(3_i64.pow(inputs.len() as u32 - 1)))
        .map(|i| {
            inputs
                .iter()
                .skip(1)
                .enumerate()
                .fold(inputs[0], |acc, (pos, v)| {
                    match get_operation(i, pos as u32) {
                        Op::Add => acc + v,
                        Op::Mult => acc * v,
                        Op::Concat => format!("{}{}", acc, v).parse().unwrap(),
                    }
                })
        })
        .any(|v| v == target)
}

fn get_operation(i: i64, pos: u32) -> Op {
    match (i / 3_i64.pow(pos)) % 3 {
        0 => Op::Add,
        1 => Op::Mult,
        2 => Op::Concat,
        _ => unreachable!(),
    }
}

pub fn seven(data: String, _: i64) -> i64 {
    data.split("\n")
        .map(|l| {
            let parts: Vec<&str> = l.split(": ").collect();
            let target = parts[0].parse().unwrap();
            let inputs: Vec<i64> = parts[1].split(" ").map(|i| i.parse().unwrap()).collect();

            if check_calibration(target, inputs) {
                target
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug)]
struct Map {
    obstacles: HashSet<(i64, i64)>,
    visited: HashMap<(i64, i64), HashSet<Direction>>,

    x: i64,
    y: i64,

    position: (i64, i64),
    facing: Direction,
}

impl Map {
    fn new(raw_map: &String) -> Self {
        let lines: Vec<&str> = raw_map.split("\n").collect();

        let mut obstacles = HashSet::new();
        let mut position = (0, 0);
        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == '#' {
                    obstacles.insert((i as i64, j as i64));
                } else if char == '^' {
                    position = (i as i64, j as i64);
                }
            }
        }

        Self {
            obstacles,
            visited: HashMap::new(),
            x: lines[0].len() as i64,
            y: lines.len() as i64,
            position,
            facing: Direction::Up,
        }
    }

    fn insert_visited(&mut self, position: (i64, i64), facing: Direction) {
        if self.visited.contains_key(&position) {
            self.visited.get_mut(&position).unwrap().insert(facing);
        } else {
            let mut set = HashSet::new();
            set.insert(facing);
            self.visited.insert(position, set);
        }
    }

    fn next(&mut self) -> bool {
        self.insert_visited(self.position, self.facing);

        let next_position = self.facing.get_index(self.position.0, self.position.1, 1);
        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= self.x
            || next_position.1 >= self.y
        {
            false
        } else if self.obstacles.contains(&next_position) {
            self.facing = self.facing.turn_right_90();

            true
        } else {
            self.position = next_position;

            true
        }
    }

    fn visited(&self, position: (i64, i64), facing: Direction) -> bool {
        match self.visited.get(&position) {
            Some(s) => s.contains(&facing),
            None => false,
        }
    }

    fn try_obstacle(&mut self, obstacle_position: (i64, i64)) -> bool {
        self.obstacles.insert(obstacle_position);

        while self.next() {
            if self.visited(self.position, self.facing) {
                return true;
            }
        }

        false
    }
}

pub fn six(data: String, part: i64) -> i64 {
    let mut map = Map::new(&data);
    let initial_position = map.position;

    if part == 1 {
        while map.next() {}
        map.visited.len() as i64
    } else {
        while map.next() {}
        map.visited.remove(&initial_position);

        map.visited
            .iter()
            .map(|v| {
                let mut map = Map::new(&data);
                map.try_obstacle(*v.0)
            })
            .fold(0, |acc, v| if v { acc + 1 } else { acc })
    }
}

#[derive(PartialEq, Eq)]
struct PageNumber<'a> {
    number: i64,
    orderings: &'a HashMap<i64, Vec<i64>>,
    reverse_orderings: &'a HashMap<i64, Vec<i64>>,
}

impl<'a> PartialOrd for PageNumber<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(o) = self.orderings.get(&self.number) {
            if o.contains(&other.number) {
                return Some(std::cmp::Ordering::Less);
            }
        }

        if let Some(o) = self.reverse_orderings.get(&self.number) {
            if o.contains(&other.number) {
                return Some(std::cmp::Ordering::Greater);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl<'a> Ord for PageNumber<'a> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> std::fmt::Debug for PageNumber<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

pub fn five(data: String, part: i64) -> i64 {
    let input: Vec<&str> = data.split("\n\n").collect();

    let mut orderings = HashMap::<i64, Vec<i64>>::new();
    let mut reverse_orderings = HashMap::<i64, Vec<i64>>::new();

    for ordering in input[0].split("\n") {
        let parts: Vec<i64> = ordering.split("|").map(|s| s.parse().unwrap()).collect();
        if orderings.contains_key(&parts[0]) {
            orderings.get_mut(&parts[0]).unwrap().push(parts[1]);
        } else {
            orderings.insert(parts[0], vec![parts[1]]);
        }

        if reverse_orderings.contains_key(&parts[1]) {
            reverse_orderings.get_mut(&parts[1]).unwrap().push(parts[0]);
        } else {
            reverse_orderings.insert(parts[1], vec![parts[0]]);
        }
    }

    let mut pages: Vec<Vec<PageNumber>> = input[1]
        .split("\n")
        .map(|s| {
            s.split(",")
                .map(|i| PageNumber {
                    number: i.parse().unwrap(),
                    orderings: &orderings,
                    reverse_orderings: &reverse_orderings,
                })
                .collect()
        })
        .collect();

    if part == 1 {
        pages
            .iter()
            .map(|p| {
                if p.windows(2).all(|w| w[0] <= w[1]) {
                    p[p.len() / 2].number
                } else {
                    0
                }
            })
            .sum()
    } else {
        pages
            .iter_mut()
            .filter(|p| !p.windows(2).all(|w| w[0] <= w[1]))
            .map(|p| {
                p.sort();
                p[p.len() / 2].number
            })
            .sum()
    }
}

static DIRECTIONS: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

static X_PAIRS: [(Direction, Direction); 4] = [
    (Direction::UpLeft, Direction::DownRight),
    (Direction::DownRight, Direction::UpLeft),
    (Direction::UpRight, Direction::DownLeft),
    (Direction::DownLeft, Direction::UpRight),
];

struct WordSearch {
    board: Vec<char>,

    x: usize,
    y: usize,
}

impl WordSearch {
    fn new(raw_board: String) -> Self {
        let rows: Vec<&str> = raw_board.split("\n").collect();
        let t = rows.iter().map(|r| r.chars()).flat_map(|c| c).collect();

        Self {
            board: t,
            x: rows.len(),
            y: rows[0].len(),
        }
    }

    fn check_square_xmas(&self, x: i64, y: i64) -> i64 {
        if self[(x, y)] != 'X' {
            return 0;
        }

        DIRECTIONS
            .iter()
            .map(|d| {
                self[d.get_index(x, y, 1)] == 'M'
                    && self[d.get_index(x, y, 2)] == 'A'
                    && self[d.get_index(x, y, 3)] == 'S'
            })
            .fold(0, |acc, b| if b { acc + 1 } else { acc })
    }

    fn check_square_mas_x(&self, x: i64, y: i64) -> i64 {
        if self[(x, y)] != 'A' {
            return 0;
        }

        let crossing = X_PAIRS
            .iter()
            .map(|(d1, d2)| {
                self[d1.get_index(x, y, 1)] == 'M' && self[d2.get_index(x, y, 1)] == 'S'
            })
            .fold(0, |acc, b| if b { acc + 1 } else { acc });

        if crossing == 2 {
            1
        } else {
            0
        }
    }

    fn check_squares(&self) -> i64 {
        (0..self.x as i64)
            .map(|x| {
                (0..self.y as i64)
                    .map(|y| self.check_square_xmas(x, y))
                    .sum::<i64>()
            })
            .sum()
    }

    fn check_squares_mas_x(&self) -> i64 {
        (0..self.x as i64)
            .map(|x| {
                (0..self.y as i64)
                    .map(|y| self.check_square_mas_x(x, y))
                    .sum::<i64>()
            })
            .sum()
    }
}

impl Index<(i64, i64)> for WordSearch {
    type Output = char;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        let x = match usize::try_from(index.0) {
            Ok(x) => x,
            Err(_) => return &'e',
        };
        let y = match usize::try_from(index.1) {
            Ok(y) => y,
            Err(_) => return &'e',
        };

        if x >= self.x || y >= self.y {
            return &'e';
        }

        &self.board[(x * self.y) + y]
    }
}

pub fn four(data: String, part: i64) -> i64 {
    let wordsearch = WordSearch::new(data);

    if part == 1 {
        wordsearch.check_squares()
    } else {
        wordsearch.check_squares_mas_x()
    }
}

#[derive(Debug)]
enum MulParserState {
    Start,
    M,
    U,
    L,
    LeftParen,
    Left(String),
    Comma,
    Right(String),
    D,
    O,
    N,
    Apostrophe,
}

#[derive(Debug)]
struct MulParser {
    data: Vec<char>,

    position: usize,
    state: MulParserState,
    mul_enabled: bool,

    left: i64,
}

impl MulParser {
    fn new(data: Vec<char>) -> Self {
        Self {
            data,
            position: 0,
            state: MulParserState::Start,
            mul_enabled: true,
            left: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.data.len()
    }

    fn next(&mut self) -> Option<i64> {
        match &self.state {
            MulParserState::Start => match self.data[self.position] {
                'm' => self.state = MulParserState::M,
                'd' => self.state = MulParserState::D,
                _ => self.state = MulParserState::Start,
            },
            MulParserState::M => match self.data[self.position] {
                'u' => self.state = MulParserState::U,
                _ => self.state = MulParserState::Start,
            },
            MulParserState::U => match self.data[self.position] {
                'l' => self.state = MulParserState::L,
                _ => self.state = MulParserState::Start,
            },
            MulParserState::L => match self.data[self.position] {
                '(' => self.state = MulParserState::LeftParen,
                _ => self.state = MulParserState::Start,
            },
            MulParserState::LeftParen => match self.data[self.position] {
                c if c.is_numeric() => self.state = MulParserState::Left(c.to_string()),
                _ => self.state = MulParserState::Start,
            },
            MulParserState::Left(s) => match self.data[self.position] {
                ',' => {
                    self.left = s.parse().unwrap();
                    self.state = MulParserState::Comma;
                }
                c if c.is_numeric() => {
                    self.state = MulParserState::Left(s.to_owned() + &c.to_string())
                }
                _ => self.state = MulParserState::Start,
            },
            MulParserState::Comma => match self.data[self.position] {
                c if c.is_numeric() => self.state = MulParserState::Right(c.to_string()),
                _ => self.state = MulParserState::Start,
            },
            MulParserState::Right(s) => match self.data[self.position] {
                ')' => {
                    let right: i64 = s.parse().unwrap();
                    self.state = MulParserState::Start;
                    self.position += 1;

                    return Some(self.left * right);
                }
                c if c.is_numeric() => {
                    self.state = MulParserState::Right(s.to_owned() + &c.to_string())
                }
                _ => self.state = MulParserState::Start,
            },
            MulParserState::D => match self.data[self.position] {
                'o' => self.state = MulParserState::O,
                _ => self.state = MulParserState::Start,
            },
            MulParserState::O => match self.data[self.position] {
                'n' => self.state = MulParserState::N,
                '(' => {
                    if self.data[self.position + 1] == ')' {
                        self.mul_enabled = true;
                    }
                }
                _ => self.state = MulParserState::Start,
            },
            MulParserState::N => match self.data[self.position] {
                '\'' => self.state = MulParserState::Apostrophe,
                _ => self.state = MulParserState::Start,
            },
            MulParserState::Apostrophe => match self.data[self.position] {
                't' => {
                    if self.data[self.position + 1] == '(' && self.data[self.position + 2] == ')' {
                        self.mul_enabled = false;
                    }
                }
                _ => self.state = MulParserState::Start,
            },
        };

        self.position += 1;
        None
    }
}

pub fn three(data: String, part: i64) -> i64 {
    let chars = data.chars().collect();
    let mut parser = MulParser::new(chars);

    let mut total = 0;
    while !parser.is_at_end() {
        if let Some(mul) = parser.next() {
            if part == 1 || parser.mul_enabled {
                total += mul;
            }
        }
    }

    total
}

pub fn two(data: String, part: i64) -> i64 {
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

    data.split("\n")
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(safe)
        .fold(0, |acc, b| if b { acc + 1 } else { acc })
}

pub fn one(data: String, part: i64) -> i64 {
    let mut locations: (Vec<i64>, Vec<i64>) = data
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
