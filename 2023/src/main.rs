use std::{fs::File, io::Read, path::Path};

fn main() {
    day_one();
}

#[allow(dead_code)]
fn day_one() {
    let data = read_file_to_string(Path::new("data/1.txt"));

    let count: u32 = data
        .lines()
        .map(|l| {
            let mut parser = CalibrationParser::new(l.chars().collect());
            let nums = parser.parse();

            format!("{}{}", nums[0], nums[nums.len() - 1])
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("{count}");
}

struct CalibrationParser {
    data: Vec<char>,
    postition: usize,
    lookahead: usize,
}

impl CalibrationParser {
    fn new(input: Vec<char>) -> Self {
        Self {
            data: input,
            postition: 0,
            lookahead: 0,
        }
    }

    fn parse(&mut self) -> Vec<char> {
        let mut nums = vec![];

        while !self.at_end() {
            let (mut next_state, mut outcome) = CalibrationParserState::None.next(self.next());
            self.lookahead += 1;

            if !outcome && next_state != CalibrationParserState::None {
                while !outcome && next_state != CalibrationParserState::None && !self.at_end() {
                    (next_state, outcome) = next_state.next(self.next());
                    self.lookahead += 1;
                }
            }

            if outcome {
                nums.push(next_state.to_num());
            }

            self.postition += 1;
            self.lookahead = 0;
        }

        nums
    }

    fn next(&self) -> char {
        self.data[self.postition + self.lookahead]
    }

    fn at_end(&self) -> bool {
        self.postition + self.lookahead >= self.data.len()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum CalibrationParserState {
    None,
    Number(char),
    One(usize),
    TwoThree,
    Two(usize),
    Three(usize),
    FourFive,
    Four(usize),
    Five(usize),
    SixSeven,
    Six(usize),
    Seven(usize),
    Eight(usize),
    Nine(usize),
}

impl CalibrationParserState {
    fn next(&self, next: char) -> (Self, bool) {
        match self {
            CalibrationParserState::None => match next {
                '0'..='9' => (Self::Number(next), true),
                'o' => (Self::One(1), false),
                't' => (Self::TwoThree, false),
                'f' => (Self::FourFive, false),
                's' => (Self::SixSeven, false),
                'e' => (Self::Eight(1), false),
                'n' => (Self::Nine(1), false),
                _ => (Self::None, false),
            },
            CalibrationParserState::Number(_) => (Self::Number(next), true),
            CalibrationParserState::One(p) => match p {
                1 => {
                    if next == 'n' {
                        (Self::One(2), false)
                    } else {
                        (Self::None, false)
                    }
                }
                2 => {
                    if next == 'e' {
                        (Self::One(3), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::TwoThree => match next {
                'w' => (Self::Two(2), false),
                'h' => (Self::Three(2), false),
                _ => (Self::None, false),
            },
            CalibrationParserState::Two(p) => match p {
                2 => {
                    if next == 'o' {
                        (Self::Two(3), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::Three(p) => match p {
                2 => {
                    if next == 'r' {
                        (Self::Three(3), false)
                    } else {
                        (Self::None, false)
                    }
                }
                3 => {
                    if next == 'e' {
                        (Self::Three(4), false)
                    } else {
                        (Self::None, false)
                    }
                }
                4 => {
                    if next == 'e' {
                        (Self::Three(5), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::FourFive => match next {
                'o' => (Self::Four(2), false),
                'i' => (Self::Five(2), false),
                _ => (Self::None, false),
            },
            CalibrationParserState::Four(p) => match p {
                2 => {
                    if next == 'u' {
                        (Self::Four(3), false)
                    } else {
                        (Self::None, false)
                    }
                }
                3 => {
                    if next == 'r' {
                        (Self::Four(4), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::Five(p) => match p {
                2 => {
                    if next == 'v' {
                        (Self::Five(3), false)
                    } else {
                        (Self::None, false)
                    }
                }
                3 => {
                    if next == 'e' {
                        (Self::Five(4), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::SixSeven => match next {
                'i' => (Self::Six(2), false),
                'e' => (Self::Seven(2), false),
                _ => (Self::None, false),
            },
            CalibrationParserState::Six(p) => match p {
                2 => {
                    if next == 'x' {
                        (Self::Six(3), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::Seven(p) => match p {
                2 => {
                    if next == 'v' {
                        (Self::Seven(3), false)
                    } else {
                        (Self::None, false)
                    }
                }
                3 => {
                    if next == 'e' {
                        (Self::Seven(4), false)
                    } else {
                        (Self::None, false)
                    }
                }
                4 => {
                    if next == 'n' {
                        (Self::Seven(5), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::Eight(p) => match p {
                1 => {
                    if next == 'i' {
                        (Self::Eight(2), false)
                    } else {
                        (Self::None, false)
                    }
                }
                2 => {
                    if next == 'g' {
                        (Self::Eight(3), false)
                    } else {
                        (Self::None, false)
                    }
                }
                3 => {
                    if next == 'h' {
                        (Self::Eight(4), false)
                    } else {
                        (Self::None, false)
                    }
                }
                4 => {
                    if next == 't' {
                        (Self::Eight(5), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
            CalibrationParserState::Nine(p) => match p {
                1 => {
                    if next == 'i' {
                        (Self::Nine(2), false)
                    } else {
                        (Self::None, false)
                    }
                }
                2 => {
                    if next == 'n' {
                        (Self::Nine(3), false)
                    } else {
                        (Self::None, false)
                    }
                }
                3 => {
                    if next == 'e' {
                        (Self::Nine(4), true)
                    } else {
                        (Self::None, false)
                    }
                }
                _ => unreachable!(),
            },
        }
    }

    fn to_num(&self) -> char {
        match self {
            CalibrationParserState::Number(c) => *c,
            CalibrationParserState::One(_) => '1',
            CalibrationParserState::Two(_) => '2',
            CalibrationParserState::Three(_) => '3',
            CalibrationParserState::Four(_) => '4',
            CalibrationParserState::Five(_) => '5',
            CalibrationParserState::Six(_) => '6',
            CalibrationParserState::Seven(_) => '7',
            CalibrationParserState::Eight(_) => '8',
            CalibrationParserState::Nine(_) => '9',
            _ => unreachable!(),
        }
    }
}

fn read_file_to_string(path: &Path) -> String {
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut data = String::new();
    match f.read_to_string(&mut data) {
        Ok(_) => {}
        Err(e) => panic!("Error reading file: {}", e),
    };

    data
}
