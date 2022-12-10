use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
    path::Path,
    rc::Rc,
};

fn main() {
    day_ten();
}

fn day_ten() {
    let data = read_file_to_string(Path::new("data/10.txt"));

    let mut current: isize = 1;
    let mut score: isize = 0;
    let mut cycle = 0;
    let mut screen = [false; 240];
    for raw_op in data.lines() {
        cycle += 1;
        let op: Vec<&str> = raw_op.split(' ').collect();
        if (cycle - 20) % 40 == 0 {
            println!("{} {}", cycle, current);
            score += current * cycle;
        }        
        if ((cycle - 1) % 40).abs_diff(current) <= 1 {
            screen[(cycle - 1) as usize] = true
        }

        match op[0] {
            "noop" => {
            },
            "addx" => {
                cycle += 1;

                if ((cycle - 1) % 40).abs_diff(current) <= 1 {
                    screen[(cycle - 1) as usize] = true
                }
                if (cycle - 20) % 40 == 0 {
                    println!("{} {}", cycle, current);
                    score += current * cycle;
                }
                current += str::parse::<isize>(op[1]).unwrap();
            },
            _ => panic!(),
        };
    }

    println!("{}", score);
    for i in 1..screen.len() + 1 {
        print!("{}", if screen[i - 1] {
            "#"
        } else {
            "."
        });

        if i % 40 == 0 {
            println!();
        }
    }
}

#[allow(dead_code)]
fn day_nine() {
    let data = read_file_to_string(Path::new("data/9.txt"));

    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut rope = vec![(0, 0); 10];

    for motion in data.lines() {
        let command: Vec<&str> = motion.split(' ').collect();
        let count: u32 = str::parse(command[1]).unwrap();
        for _ in 0..count {
            rope[0] = match command[0] {
                "R" => (rope[0].0 + 1, rope[0].1),
                "L" => (rope[0].0 - 1, rope[0].1),
                "U" => (rope[0].0, rope[0].1 + 1),
                "D" => (rope[0].0, rope[0].1 - 1),
                _ => panic!(),
            };

            for i in 1..rope.len() {
                rope[i] = move_tail(rope[i - 1], rope[i]);
            }
            visited.insert(rope[rope.len() - 1]);
        }
    }

    println!("{:?}", visited.len());
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
        return tail;
    }

    let mut out = tail;
    let diff0 = if tail.0 - head.0 > 0 { -1 } else { 1 };
    let diff1 = if tail.1 - head.1 > 0 { -1 } else { 1 };

    if head.0 != tail.0 {
        out.0 += diff0;
    }
    if head.1 != tail.1 {
        out.1 += diff1;
    }

    out
}

#[allow(dead_code)]
fn day_eight() {
    let data = read_file_to_string(Path::new("data/8.txt"));

    let mut grid = vec![];
    for (i, line) in data.lines().enumerate() {
        grid.push(vec![]);
        for item in line.chars() {
            grid[i].push(item.to_digit(10).unwrap());
        }
    }

    let mut best_scenic = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let tree = grid[i][j];

            let mut scenic = 1;
            let mut distance = 0;
            for k in 0..i {
                distance += 1;
                if grid[i - (k + 1)][j] >= tree {
                    break;
                }
            }
            scenic *= distance;
            distance = 0;

            for k in 0..j {
                distance += 1;
                if grid[i][j - (k + 1)] >= tree {
                    break;
                }
            }
            scenic *= distance;
            distance = 0;

            for k in 0..grid.len() - i - 1 {
                distance += 1;
                if grid[i + k + 1][j] >= tree {
                    break;
                }
            }
            scenic *= distance;
            distance = 0;

            for k in 0..grid[i].len() - j - 1 {
                distance += 1;
                if grid[i][j + k + 1] >= tree {
                    break;
                }
            }
            scenic *= distance;

            if scenic > best_scenic {
                best_scenic = scenic;
            }
        }
    }

    println!("{:?}", best_scenic);
}

#[allow(dead_code)]
fn day_seven() {
    let data = read_file_to_string(Path::new("data/7.txt"));

    let root = PuzzleDir {
        name: "/".to_string(),
        dirs: vec![],
        files: vec![],
        parent: None,
    };
    let mut current = Rc::new(RefCell::new(root));
    let root = current.clone();

    for line in data.split("\n").skip(1) {
        if line.chars().nth(0).unwrap() == '$' {
            let command: Vec<&str> = line.split(" ").skip(1).collect();
            if command[0] == "cd" {
                if command[1] != ".." {
                    let t = current
                        .borrow()
                        .dirs
                        .iter()
                        .find(|d| d.borrow().name == command[1])
                        .expect("failed to find dir")
                        .clone();

                    current = t;
                } else {
                    let t = current.borrow().parent.clone();
                    current = t.unwrap();
                }
            }
        } else {
            let output: Vec<&str> = line.split(" ").collect();
            if output[0] == "dir" {
                let dir = PuzzleDir {
                    name: output[1].to_string(),
                    dirs: vec![],
                    files: vec![],
                    parent: Some(current.clone()),
                };

                current.borrow_mut().dirs.push(Rc::new(RefCell::new(dir)));
            } else {
                let file = PuzzleFile {
                    size: str::parse(output[0]).unwrap(),
                };

                current.borrow_mut().files.push(file);
            }
        }
    }

    println!("{}", root.borrow().add_size(100_000));

    let to_find = root.borrow().size() - 40000000;
    println!("{}", root.borrow().find_to_delete(to_find));
}

#[derive(Clone, Debug)]
struct PuzzleFile {
    size: usize,
}

#[derive(Clone, Debug)]
struct PuzzleDir {
    name: String,
    dirs: Vec<Rc<RefCell<PuzzleDir>>>,
    files: Vec<PuzzleFile>,
    parent: Option<Rc<RefCell<PuzzleDir>>>,
}

impl PuzzleDir {
    fn size(&self) -> usize {
        let score: usize = self.files.iter().map(|f| f.size).sum();

        score
            + self
                .dirs
                .iter()
                .map(|dir| dir.borrow().size())
                .sum::<usize>()
    }

    fn find_to_delete(&self, threshold: usize) -> usize {
        let mut current = if self.size() >= threshold {
            self.size()
        } else {
            usize::MAX
        };
        for dir in self.dirs.iter() {
            let to_del = dir.borrow().find_to_delete(threshold);
            if to_del < current {
                current = to_del;
            }
        }

        current
    }

    fn add_size(&self, threshold: usize) -> usize {
        let mut size = 0;
        if self.size() < 100_000 {
            size += self.size();
        }

        for dir in self.dirs.iter() {
            size += dir.borrow().add_size(threshold);
        }

        size
    }
}

#[allow(dead_code)]
fn day_six() {
    let data = read_file_to_string(Path::new("data/6.txt"));

    for (i, window) in data.as_bytes().windows(14).enumerate() {
        let mut set = HashSet::new();
        for item in window {
            set.insert(*item);
        }
        if set.len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}

#[allow(dead_code)]
fn day_five() {
    let data = read_file_to_string(Path::new("data/5.txt"));

    let mut stacks: [VecDeque<char>; 9] = Default::default();
    let mut lines = data.split("\n");
    while let Some(line) = lines.next() {
        if line.chars().nth(1).unwrap().is_numeric() {
            break;
        }

        for (i, char) in line.chars().enumerate() {
            if char == '[' || char == ']' || char == ' ' {
                continue;
            }

            stacks[(i - 1) / 4].push_back(char);
        }
    }
    lines.next();

    for line in lines {
        let crane_move = CraneMove::parse(line);
        let items: Vec<char> = stacks[crane_move.src].drain(0..crane_move.count).collect();
        for item in items.iter().rev() {
            stacks[crane_move.dest].push_front(*item);
        }
    }

    let answer = stacks.map(|stack| stack.front().unwrap().clone());

    println!("{:?}", answer);
}

#[derive(Debug)]
struct CraneMove {
    count: usize,
    src: usize,
    dest: usize,
}

impl CraneMove {
    fn parse(raw: &str) -> Self {
        let mut input = [0; 3];
        for (i, item) in raw.split(" ").skip(1).step_by(2).enumerate() {
            input[i] = str::parse(item).unwrap();
        }

        CraneMove {
            count: input[0],
            src: input[1] - 1,
            dest: input[2] - 1,
        }
    }
}

#[allow(dead_code)]
fn day_four() {
    let data = read_file_to_string(Path::new("data/4.txt"));

    let score = data
        .split("\n")
        .map(|assign| {
            let assignments: Vec<Interval> = assign
                .split(",")
                .map(|assign| Interval::parse(assign))
                .collect();

            println!("{:?}", assignments);

            (
                (assignments[0].start <= assignments[1].end
                    && assignments[0].end >= assignments[1].end)
                    || (assignments[1].start <= assignments[0].start
                        && assignments[1].end >= assignments[0].end),
                (assignments[0].start <= assignments[1].end
                    && assignments[0].end >= assignments[1].start)
                    || (assignments[1].start <= assignments[0].end
                        && assignments[1].end >= assignments[0].start),
            )
        })
        .fold((0, 0), |acc, overlap| {
            if overlap.0 && overlap.1 {
                (acc.0 + 1, acc.1 + 1)
            } else if overlap.1 {
                (acc.0, acc.1 + 1)
            } else if overlap.0 {
                (acc.0 + 1, acc.1)
            } else {
                acc
            }
        });

    println!("{:?}", score);
}

#[derive(Debug)]
struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    fn parse(input: &str) -> Self {
        let interval: Vec<&str> = input.split("-").collect();
        Interval {
            start: str::parse(interval[0]).unwrap(),
            end: str::parse(interval[1]).unwrap(),
        }
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

    #[allow(dead_code)]
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

#[allow(dead_code)]
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
