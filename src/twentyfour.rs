use std::{
    collections::HashMap,
    fs::{self},
    ops::Index,
    path::Path,
};

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn get_index(&self, x: i64, y: i64, n: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x - n, y),
            Direction::Down => (x + n, y),
            Direction::UpRight => (x - n, y + n),
            Direction::UpLeft => (x - n, y - n),
            Direction::DownRight => (x + n, y + n),
            Direction::DownLeft => (x + n, y - n),
            Direction::Right => (x, y + n),
            Direction::Left => (x, y - n),
        }
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

    fn check_square(&self, x: i64, y: i64) -> i64 {
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

    fn check_squares(&self) -> i64 {
        (0..self.x as i64)
            .map(|x| {
                (0..self.y as i64)
                    .map(|y| self.check_square(x, y))
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

pub fn four(data: &Path, part: i64) -> i64 {
    let wordsearch =
        WordSearch::new(fs::read_to_string(data).expect("should be able to read data"));

    wordsearch.check_squares()
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

pub fn three(data: &Path, part: i64) -> i64 {
    let chars = fs::read_to_string(data)
        .expect("should be able to read data")
        .chars()
        .collect();
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
