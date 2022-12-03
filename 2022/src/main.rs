use std::{fs::File, io::Read, path::Path, collections::HashSet};

fn main() {
    day_one();
    day_two();
    day_three();
}

fn day_three() {
    let data = read_file_to_string(Path::new("data/3.txt"));

    let mut common = vec![];
    for bag in data.split("\n") {
        let (one, two) = bag.split_at(bag.len() / 2);
        let mut items = HashSet::new();
        for item in one.chars() {
            items.insert(item);
        }

        let mut added = HashSet::new();
        for item in two.chars() {
            if items.contains(&item) && !added.contains(&item) {
                common.push(item);
                added.insert(item);
            }
        }
    }

    let score = common.iter().fold(0, |acc, item| {
        if item.is_uppercase() {
            acc + *item as u32 - 38
        } else {
            acc + *item as u32 - 96
        }
    });

    println!("{}", score);

    let collect: Vec<&str> = data.split("\n").collect();
    let mut badges = vec![];
    for group in collect.chunks(3) {
        let mut common = vec![HashSet::new(), HashSet::new(), HashSet::new()];
        for i in 0..3 {
            for item in group[i].chars() {
                common[i].insert(item);
            }
        }

        let first = common[0].intersection(&common[1]).copied().collect();
        let badge = common[2].intersection(&first).next().unwrap();
        badges.push(badge.clone());
    }
    let score = badges.iter().fold(0, |acc, item| {
        if item.is_uppercase() {
            acc + *item as u32 - 38
        } else {
            acc + *item as u32 - 96
        }
    });


    println!("{:?}", score);
}

fn day_two() {
    let data = read_file_to_string(Path::new("data/2.txt"));

    let mut score = 0;
    for round in data.split("\n") {
        let mut r = round.split(" ");
        let opponent = Move::parse(r.next().unwrap());
        let outcome = Outcome::parse(r.next().unwrap());
        score += outcome.play_round(&opponent);
    }

    println!("{}", score);
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn parse(raw_move: &str) -> Self {
        match raw_move {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("unexpected move type"),
        }
    }

    fn play_round(&self, other: &Move) -> i32 {
        match self {
            Self::Win => match other {
                Move::Rock => 8,
                Move::Paper => 9,
                Move::Scissors => 7,
            },
            Self::Lose => match other {
                Move::Rock => 3,
                Move::Paper => 1,
                Move::Scissors => 2,
            },
            Self::Draw => match other {
                Move::Rock => 4,
                Move::Paper => 5,
                Move::Scissors => 6,
            },
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(raw_move: &str) -> Self {
        match raw_move {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("unexpected move type"),
        }
    }

    fn play_round(&self, other: &Move) -> i32 {
        match self {
            Move::Rock => match other {
                Move::Rock => 4,
                Move::Paper => 1,
                Move::Scissors => 7,
            },
            Move::Paper => match other {
                Move::Rock => 8,
                Move::Paper => 5,
                Move::Scissors => 2,
            },
            Move::Scissors => match other {
                Move::Rock => 3,
                Move::Paper => 9,
                Move::Scissors => 6,
            },
        }
    }
}

fn day_one() {
    let data = read_file_to_string(Path::new("data/1.txt"));

    let lines = data.split("\n");
    let mut calories = vec![0];

    for line in lines {
        if line.trim() != "" {
            if let Some(calorie) = calories.last_mut() {
                *calorie += line
                    .trim()
                    .parse::<u32>()
                    .expect("failed to parse data as int")
            }
        } else {
            calories.push(0);
        }
    }

    calories.sort_by(|a, b| b.cmp(a));

    println!("{:?}", calories[0] + calories[1] + calories[2]);
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
