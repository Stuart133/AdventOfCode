use std::{collections::HashMap, collections::HashSet, fs::File, io::Read, path::Path};

fn main() {
    day_seven();
}

#[allow(dead_code)]
fn day_seven() {
    let data = read_file_to_string(Path::new("data/7.txt"));

    let mut games: Vec<CamelGame> = data
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let hand = Hand::construct(&parts[0].chars().map(|c| Card::parse(c)).collect());

            CamelGame {
                hand,
                bid: parts[1].parse().unwrap(),
            }
        })
        .collect();

    games.sort_by(|a, b| b.hand.cmp(&a.hand));
    let score = games
        .iter()
        .inspect(|g| println!("{:?}", g))
        .enumerate()
        .fold(0, |acc, (i, g)| acc + ((i + 1) * g.bid as usize));
    println!("{:?}", score)
}

#[derive(Debug)]
struct CamelGame {
    hand: Hand,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    FiveOfAKind(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
}

impl Hand {
    fn construct(cards: &Vec<Card>) -> Self {
        let mut equal = HashMap::new();
        for card in cards {
            if equal.contains_key(card) {
                *equal.get_mut(card).unwrap() += 1;
            } else {
                equal.insert(card.clone(), 1);
            }
        }

        let jokers = match equal.get(&Card::Joker) {
            Some(v) => *v,
            None => 0,
        };
        let mut t = vec![vec![]; 10];

        for (k, v) in equal {
            if k != Card::Joker {
                t[v - 1].push(k.clone());
            }
        }

        println!("{:?}", t);

        if jokers == 5 || t[4 - jokers].len() >= 1 {
            Self::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
        } else if jokers == 4 || t[3 - jokers].len() >= 1 {
            Self::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
        } else if (jokers == 3 && t[1].len() >= 1)
            || (jokers == 2 && t[2].len() >= 1)
            || (jokers == 1 && t[1].len() == 2)
            || (t[2].len() == 1 && t[1].len() == 1)
        {
            Self::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4])
        } else if jokers == 3 || t[2 - jokers].len() >= 1 {
            Self::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
        } else if jokers == 2 && t[1].len() >= 1
            || jokers == 2 && t[0].len() >= 2
            || jokers == 1 && t[1].len() >= 1 && t[0].len() >= 1
            || t[1].len() == 2
        {
            Self::TwoPair(cards[0], cards[1], cards[2], cards[3], cards[4])
        } else if jokers == 2 || t[1 - jokers].len() >= 1 {
            Self::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4])
        } else {
            Self::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4])
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Joker,
}

impl Card {
    fn parse(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            '1' => Self::One,
            'J' => Self::Joker,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
fn day_six_2() {
    let data = read_file_to_string(Path::new("data/6.txt"));

    let data: Vec<u64> = data
        .lines()
        .map(|l| {
            let t = l
                .split(' ')
                .filter(|n| !n.is_empty())
                .fold(String::new(), |acc, s| acc + s);

            t.parse().unwrap()
        })
        .collect();

    let race = Race {
        time: data[0],
        distance: data[1],
    };

    println!("{:?}", race.ways_to_win());
}

#[allow(dead_code)]
fn day_six() {
    let data = read_file_to_string(Path::new("data/6.txt"));

    let data: Vec<Vec<u64>> = data
        .lines()
        .map(|l| {
            l.split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let races = data[0]
        .iter()
        .zip(&data[1])
        .map(|(a, b)| Race {
            time: *a,
            distance: *b,
        })
        .map(|r| r.ways_to_win())
        .fold(1, |acc, ways| acc * ways);

    println!("{:?}", races);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        (0..self.time)
            .map(|t| t * (self.time - t))
            .map(|d| d > self.distance)
            .filter(|p| *p)
            .count()
    }
}

#[allow(dead_code)]
fn day_five() {
    let data = read_file_to_string(Path::new("data/5_smol.txt"));
    let raw_seeds = "79 14 55 13";

    let mut seeds: Vec<SeedInterval> = raw_seeds
        .split(' ')
        .zip(raw_seeds.split(' ').skip(1))
        .step_by(2)
        .map(|(ns, count)| SeedInterval {
            start: ns.parse().unwrap(),
            width: count.parse().unwrap(),
        })
        .collect();
    let mut seeds_tmp = vec![];

    data.split("\n\n")
        .map(|m| AlmanacMap {
            entries: m
                .lines()
                .map(|l| {
                    l.split(' ')
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<i64>>()
                })
                .map(|ns| Interval {
                    start: ns[1],
                    width: ns[2],
                    offset: ns[0] - ns[1],
                })
                .collect(),
        })
        .for_each(|m| {
            for seed in seeds.iter_mut() {
                let mut t = m.transform_interval(seed);
                seeds_tmp.append(&mut t);
            }

            seeds = seeds_tmp.clone();
            seeds_tmp.clear();

            println!("one map done");
        });

    println!("{:?}", seeds.iter().map(|interval| interval.start).min());
}

#[derive(Debug, Clone)]
struct SeedInterval {
    start: i64,
    width: i64,
}

#[derive(Debug)]
struct AlmanacMap {
    entries: Vec<Interval>,
}

impl AlmanacMap {
    fn transform_interval(&self, seeds: &SeedInterval) -> Vec<SeedInterval> {
        let mut new_intervals = vec![];
        let mut start = self.transform(seeds.start);
        let mut offset = 0;
        let mut previous = self.transform(seeds.start) - 1;

        for seed in seeds.start..=seeds.start + seeds.width {
            let new_seed = self.transform(seed);

            // We need to start a new interval
            if new_seed - 1 != previous {
                new_intervals.push(SeedInterval {
                    start,
                    width: offset - 1,
                });

                start = new_seed;
                offset = 1;
            } else {
                offset += 1;
            }

            previous = new_seed;
        }

        new_intervals.push(SeedInterval {
            start,
            width: offset - 1,
        });

        new_intervals
    }

    fn transform(&self, n: i64) -> i64 {
        for entry in self.entries.iter() {
            match entry.within(n) {
                Some(new) => return new,
                None => {}
            }
        }

        n
    }
}

#[derive(Debug)]
struct Interval {
    start: i64,
    width: i64,
    offset: i64,
}

impl Interval {
    fn within(&self, n: i64) -> Option<i64> {
        if n >= self.start && n <= self.start + self.width {
            Some(n + self.offset)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn day_four() {
    let data = read_file_to_string(Path::new("data/4.txt"));

    let initial_cards: Vec<CardGame> = data
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let t = l.split(':').collect::<Vec<&str>>()[1];
            let numbers: Vec<HashSet<u32>> = t
                .split('|')
                .map(|ns| {
                    let numbers: HashSet<u32> = ns
                        .trim()
                        .split(' ')
                        .map(|n| n.trim())
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse().unwrap())
                        .collect();

                    numbers
                })
                .collect();

            CardGame {
                winning_numbers: numbers[0].clone(),
                numbers: numbers[1].clone(),
                index: i,
            }
        })
        .collect();

    let mut running_total = vec![1u32; initial_cards.len()];

    for (i, game) in initial_cards.iter().enumerate() {
        let card_count = running_total[i];

        running_total[game.index + 1..(game.index + 1 + game.winning())]
            .iter_mut()
            .for_each(|x| {
                *x += card_count;
            });
    }

    let total = running_total.iter().fold(0, |acc, x| acc + x);

    println!("{}", total);
}

#[derive(Clone)]
struct CardGame {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
    index: usize,
}

impl CardGame {
    fn winning(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }
}

#[allow(dead_code)]
fn day_three() {
    let data = read_file_to_string(Path::new("data/3.txt"));

    let mut gears = HashMap::new();
    let mut part_numbers = vec![];

    for (i, line) in data.lines().map(|l| l.as_bytes()).enumerate() {
        let mut position = 0;
        let mut lookahead = 0;

        while position < line.len() {
            match line[position + lookahead] {
                b'.' => {
                    if lookahead == 0 {
                        position += 1;
                    } else {
                        let part_number: String = line[position..position + lookahead]
                            .iter()
                            .map(|b| char::from(*b))
                            .collect();

                        part_numbers.push(PartNumber {
                            number: part_number.parse().unwrap(),
                            top_left: (i as i32 - 1, position as i32 - 1),
                            bottom_right: (i as i32 + 1, position as i32 + lookahead as i32),
                        });

                        position += lookahead;
                        lookahead = 0;
                    }
                }
                b'0'..=b'9' => {
                    lookahead += 1;
                }
                _ => {
                    if lookahead == 0 {
                        if line[position + lookahead] == b'*' {
                            gears.insert((i as i32, position as i32), Gear { meshed: vec![] });
                        }

                        position += 1;
                    } else {
                        let part_number: String = line[position..position + lookahead]
                            .iter()
                            .map(|b| char::from(*b))
                            .collect();

                        part_numbers.push(PartNumber {
                            number: part_number.parse().unwrap(),
                            top_left: (i as i32 - 1, position as i32 - 1),
                            bottom_right: (i as i32 + 1, position as i32 + lookahead as i32),
                        });

                        position += lookahead;
                        lookahead = 0;
                    }
                }
            }
        }
    }

    println!("{:?}", gears);
    println!("{:?}", part_numbers);

    for pn in part_numbers {
        for i in pn.top_left.1..=pn.bottom_right.1 {
            // Top
            if let Some(gear) = gears.get_mut(&(pn.top_left.0, i)) {
                gear.meshed.push(pn.number);
            }
        }
        for i in pn.top_left.1..=pn.bottom_right.1 {
            // Bottom
            if let Some(gear) = gears.get_mut(&(pn.bottom_right.0, i)) {
                gear.meshed.push(pn.number);
            }
        }
        for i in pn.top_left.0 + 1..pn.bottom_right.0 {
            // Left
            if let Some(gear) = gears.get_mut(&(i, pn.top_left.1)) {
                gear.meshed.push(pn.number);
            }
        }
        for i in pn.top_left.0 + 1..pn.bottom_right.0 {
            // Right
            if let Some(gear) = gears.get_mut(&(i, pn.bottom_right.1)) {
                gear.meshed.push(pn.number);
            }
        }
    }

    let count = gears
        .iter()
        .filter(|(_, v)| v.meshed.len() == 2)
        .fold(0, |acc, (_, g)| acc + (g.meshed[0] * g.meshed[1]));

    println!("{}", count);
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

#[derive(Debug)]
struct Gear {
    meshed: Vec<u32>,
}

#[allow(dead_code)]
fn day_two() {
    let data = read_file_to_string(Path::new("data/2.txt"));

    let count = data
        .lines()
        .enumerate()
        .map(|(i, l)| Game::new(l, i + 1))
        .map(|g| g.max_blue * g.max_green * g.max_red)
        .fold(0, |acc, g| acc + g);

    println!("{count}");
}

#[derive(Debug)]
struct Game {
    _id: usize,
    max_blue: u32,
    max_red: u32,
    max_green: u32,
}

impl Game {
    fn new(line: &str, i: usize) -> Self {
        let parts: Vec<&str> = line.split(':').collect();
        let draws = parts[1].split(';').map(|r| Draw::new(r));

        let mut game = Game {
            _id: i,
            max_blue: 0,
            max_red: 0,
            max_green: 0,
        };

        for draw in draws {
            game.max_blue = game.max_blue.max(draw.blue);
            game.max_green = game.max_green.max(draw.green);
            game.max_red = game.max_red.max(draw.red);
        }

        game
    }
}

struct Draw {
    blue: u32,
    red: u32,
    green: u32,
}

impl Draw {
    fn new(raw: &str) -> Self {
        let mut output = Draw {
            blue: 0,
            red: 0,
            green: 0,
        };

        for draw in raw.split(',').map(|s| s.trim()) {
            let parts: Vec<&str> = draw.split(' ').collect();
            if parts[1].contains("red") {
                output.red = parts[0].parse().unwrap();
            } else if parts[1].contains("blue") {
                output.blue = parts[0].parse().unwrap();
            } else if parts[1].contains("green") {
                output.green = parts[0].parse().unwrap();
            }
        }

        output
    }
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
