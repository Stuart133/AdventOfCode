use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
    path::Path,
    rc::Rc,
    thread,
};

use itertools::Itertools;

fn main() {
    day_twentyone();
}

fn day_twentyone() {
    let data = read_file_to_string(Path::new("data/21.txt"));

    let monkies = HashMap::from_iter(data.lines().map(|l| {
        let parts = l.split(": ").collect_vec();

        (parts[0].to_string(), MonkeyRiddle::parse(parts[1]))
    }));
    let mut parents = HashMap::new();
    for (k, v) in monkies.iter() {
        match v {
            MonkeyRiddle::Operation(l, r, _) => {
                parents.insert(l.clone(), k.clone());
                parents.insert(r.clone(), k.clone());
            }
            MonkeyRiddle::Number(_) => {}
        }
    }

    let root = monkies.get(&"root".to_string()).unwrap().evaluate(&monkies);
    println!("{root}");

    let mut path = generate_path(&parents, &monkies);
    path.reverse();

    if let MonkeyRiddle::Operation(l, r, _) = monkies.get(&"root".to_string()).unwrap() {
        let left = monkies.get(l).unwrap().evaluate_human(&monkies);
        let right = monkies.get(r).unwrap().evaluate_human(&monkies);

        let h = if let Some(target) = left {
            solve_human(r, path, target, &monkies)
        } else if let Some(target) = right {
            solve_human(l, path, target, &monkies)
        } else {
            panic!();
        };

        println!("{h}");
    }
}

fn solve_human(
    root: &String,
    path: Vec<Dir>,
    mut target: i64,
    operations: &HashMap<String, MonkeyRiddle>,
) -> i64 {
    let mut current = operations.get(root).unwrap();

    for dir in path {
        match current {
            MonkeyRiddle::Operation(l, r, op) => {
                target = match dir {
                    Dir::Left => {
                        current = operations.get(l).unwrap();
                        let value = operations.get(r).unwrap().evaluate(operations);

                        match op {
                            Operation::Add => target - value,
                            Operation::Subtract => target + value,
                            Operation::Product => target / value,
                            Operation::Divide => target * value,
                        }
                    }
                    Dir::Right => {
                        current = operations.get(r).unwrap();
                        let value = operations.get(l).unwrap().evaluate(operations);

                        match op {
                            Operation::Add => target - value,
                            Operation::Subtract => value - target,
                            Operation::Product => target / value,
                            Operation::Divide => value / target,
                        }
                    }
                };
            }
            MonkeyRiddle::Number(_) => {}
        }
    }

    target
}

fn generate_path(
    parents: &HashMap<String, String>,
    operations: &HashMap<String, MonkeyRiddle>,
) -> Vec<Dir> {
    let mut out = vec![];
    let mut prev = &"humn".to_string();
    let mut current = parents.get("humn").unwrap();

    while *current != "root".to_string() {
        match operations.get(current).unwrap() {
            MonkeyRiddle::Operation(l, _, _) => {
                if *l == *prev {
                    out.push(Dir::Left)
                } else {
                    out.push(Dir::Right)
                }
            }
            MonkeyRiddle::Number(_) => panic!(),
        }

        prev = current;
        current = parents.get(current).unwrap();
    }

    out
}

#[derive(Clone)]
enum MonkeyRiddle {
    Operation(String, String, Operation),
    Number(i64),
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl MonkeyRiddle {
    fn parse(input: &str) -> Self {
        let parts = input.split(" ").collect_vec();
        match parts.len() {
            1 => Self::Number(str::parse(parts[0]).unwrap()),
            3 => Self::Operation(
                parts[0].to_string(),
                parts[2].to_string(),
                Operation::parse(parts[1]),
            ),
            _ => panic!("unexpected operation input"),
        }
    }

    fn evaluate(&self, operations: &HashMap<String, MonkeyRiddle>) -> i64 {
        match self {
            MonkeyRiddle::Operation(left, right, op) => {
                let left = operations.get(left).unwrap().evaluate(operations);
                let right = operations.get(right).unwrap().evaluate(operations);

                match op {
                    Operation::Add => left + right,
                    Operation::Subtract => left - right,
                    Operation::Product => left * right,
                    Operation::Divide => left / right,
                }
            }
            MonkeyRiddle::Number(n) => *n,
        }
    }

    fn evaluate_human(&self, operations: &HashMap<String, MonkeyRiddle>) -> Option<i64> {
        match self {
            MonkeyRiddle::Operation(left, right, op) => {
                if *left == "humn".to_string() || *right == "humn".to_string() {
                    return None;
                } else {
                    let left = operations.get(left).unwrap().evaluate_human(operations);
                    let right = operations.get(right).unwrap().evaluate_human(operations);

                    if left == None || right == None {
                        return None;
                    }

                    let l = left.unwrap();
                    let r = right.unwrap();

                    match op {
                        Operation::Add => Some(l + r),
                        Operation::Subtract => Some(l - r),
                        Operation::Product => Some(l * r),
                        Operation::Divide => Some(l / r),
                    }
                }
            }
            MonkeyRiddle::Number(n) => Some(*n),
        }
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Subtract,
    Product,
    Divide,
}

impl Operation {
    fn parse(input: &str) -> Self {
        match input {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Product,
            "/" => Self::Divide,
            _ => panic!("unexpected op"),
        }
    }
}

#[allow(dead_code)]
fn day_nineteen() {
    let data = read_file_to_string(Path::new("data/19.txt"));

    let mut handles = vec![];
    for (_, blueprint) in data
        .lines()
        .map(|l| {
            let values = l.split(' ').map(|n| str::parse(n).unwrap()).collect_vec();
            Blueprint {
                ore_cost: values[0],
                clay_cost: values[1],
                obsidian_cost: (values[2], values[3]),
                geode_cost: (values[4], values[5]),
            }
        })
        .enumerate()
    {
        handles.push(thread::spawn(move || {
            let mut states = HashMap::new();
            let state = State {
                ore_robots: 1,
                ore: 0,
                clay_robots: 0,
                clay: 0,
                obsidian_robots: 0,
                obsidian: 0,
                geode_robots: 0,
                geodes: 0,
                time_step: 32,
            };
            let geodes = solve_production(&blueprint, state, &mut states);
            println!("{}", geodes);
        }));
    }

    for handle in handles {
        handle.join().expect("oh no");
    }
}

fn solve_production(
    blueprint: &Blueprint,
    state: State,
    memoized: &mut HashMap<State, usize>,
) -> usize {
    match memoized.get(&state) {
        Some(value) => *value,
        None => {
            let mut best = 0;
            if state.time_step == 0 {
                return state.geodes;
            }

            if state.ore >= blueprint.geode_cost.0 && state.obsidian >= blueprint.geode_cost.1 {
                let mut new_state = state.clone();
                new_state.step_time();
                new_state.ore -= blueprint.geode_cost.0;
                new_state.obsidian -= blueprint.geode_cost.1;
                new_state.geode_robots += 1;

                best = best.max(solve_production(blueprint, new_state, memoized));
            } else if state.ore >= blueprint.obsidian_cost.0
                && state.clay >= blueprint.obsidian_cost.1
                && state.obsidian_robots <= blueprint.geode_cost.1
            {
                let mut new_state = state.clone();
                new_state.step_time();
                new_state.ore -= blueprint.obsidian_cost.0;
                new_state.clay -= blueprint.obsidian_cost.1;
                new_state.obsidian_robots += 1;

                best = best.max(solve_production(blueprint, new_state, memoized));
            } else {
                if state.ore >= blueprint.ore_cost
                    && state.ore_robots
                        <= (blueprint.ore_cost
                            + blueprint.clay_cost
                            + blueprint.obsidian_cost.0
                            + blueprint.geode_cost.0)
                {
                    let mut new_state = state.clone();
                    new_state.step_time();
                    new_state.ore -= blueprint.ore_cost;
                    new_state.ore_robots += 1;

                    best = best.max(solve_production(blueprint, new_state, memoized));
                }
                if state.ore >= blueprint.clay_cost
                    && state.clay_robots <= blueprint.obsidian_cost.1
                {
                    let mut new_state = state.clone();
                    new_state.step_time();
                    new_state.ore -= blueprint.clay_cost;
                    new_state.clay_robots += 1;

                    best = best.max(solve_production(blueprint, new_state, memoized));
                }
            }

            let mut new_state = state.clone();
            new_state.step_time();
            best = best.max(solve_production(blueprint, new_state, memoized));

            memoized.insert(state, best);

            best
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State {
    ore_robots: usize,
    ore: usize,
    clay_robots: usize,
    clay: usize,
    obsidian_robots: usize,
    obsidian: usize,
    geode_robots: usize,
    geodes: usize,
    time_step: usize,
}

impl State {
    fn step_time(&mut self) {
        self.time_step -= 1;
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
}

#[allow(dead_code)]
fn day_eighteen() {
    let data = read_file_to_string(Path::new("data/18.txt"));

    let input = data.lines().map(|l| {
        let cube: (usize, usize, usize) = l
            .split(',')
            .map(|coord| str::parse(coord).unwrap())
            .collect_tuple()
            .unwrap();

        Cube {
            x: cube.0 + 1,
            y: cube.1 + 1,
            z: cube.2 + 1,
        }
    });

    let mut cubes = HashSet::new();
    let mut surface = 0;
    let mut bounding = (0, 0, 0);
    for cube in input {
        let score: i32 = generate_test(cube.x, cube.y, cube.z)
            .iter()
            .map(|c| if cubes.contains(c) { -1 } else { 1 })
            .sum();
        surface += score;
        bounding.0 = bounding.0.max(cube.x + 1);
        bounding.1 = bounding.1.max(cube.y + 1);
        bounding.2 = bounding.2.max(cube.z + 1);
        cubes.insert(cube);
    }

    let mut visited = HashSet::new();
    for i in 1..bounding.0 {
        for j in 1..bounding.1 {
            for k in 1..bounding.2 {
                if !visited.contains(&Cube { x: i, y: j, z: k })
                    && !cubes.contains(&Cube { x: i, y: j, z: k })
                {
                    surface -= check_inside(&cubes, &mut visited, i, j, k, bounding);
                }
            }
        }
    }

    println!("{}", surface);
}

fn check_inside(
    cubes: &HashSet<Cube>,
    visited: &mut HashSet<Cube>,
    x: usize,
    y: usize,
    z: usize,
    bound: (usize, usize, usize),
) -> i32 {
    let mut queue = VecDeque::new();
    let mut internal_visited = HashSet::new();
    let mut surfaces = 0;
    queue.push_back(Cube { x, y, z });

    let mut enclosed = true;
    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();
        if internal_visited.contains(&cube) {
            continue;
        }

        for test in generate_test(cube.x, cube.y, cube.z) {
            if test.x == 0
                || test.x == bound.0
                || test.y == 0
                || test.y == bound.1
                || test.z == 0
                || test.z == bound.2
            {
                enclosed = false;
                continue;
            } else if cubes.contains(&test) {
                surfaces += 1;
            } else {
                queue.push_back(test.clone())
            }

            visited.insert(cube.clone());
            internal_visited.insert(cube.clone());
        }
    }

    if enclosed {
        println!("{} {} {} {}", x, y, z, surfaces);
        surfaces
    } else {
        0
    }
}

fn generate_test(x: usize, y: usize, z: usize) -> Vec<Cube> {
    vec![
        Cube { x: x + 1, y, z },
        Cube { x: x - 1, y, z },
        Cube { x, y: y + 1, z },
        Cube { x, y: y - 1, z },
        Cube { x, y, z: z + 1 },
        Cube { x, y, z: z - 1 },
    ]
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

#[allow(dead_code)]
fn day_seventeen() {
    let data = read_file_to_string(Path::new("data/17.txt"));

    let gas = data
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!(),
        })
        .collect_vec();

    let mut chamber: [VecDeque<bool>; 7] = Default::default();
    for col in chamber.iter_mut() {
        for _ in 0..5000 {
            col.push_back(false);
        }
    }

    let shapes = vec![
        Shape {
            points: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        Shape {
            points: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        },
        Shape {
            points: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        },
        Shape {
            points: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        Shape {
            points: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        },
    ];

    let mut start = (2, 3);
    let mut height_offset = 0;
    let mut gas_step = 0;
    let mut visited = HashMap::<CavePeriod, (usize, usize)>::new();
    let mut top_points = [0; 7];
    let mut period: Option<Period> = None;
    let mut i = 0;
    let count = 1000000000000;
    while i < count {
        let shape = shapes[i % shapes.len()].clone();
        let mut position = start.clone();

        if let Some(period) = &period {
            if i + (period.step_change * 1000) < count {
                i += period.step_change * 1000;
                height_offset += period.height_change * 1000;

                continue;
            } else if i + (period.step_change * 100) < count {
                i += period.step_change * 100;
                height_offset += period.height_change * 100;

                continue;
            } else if i + period.step_change < count {
                i += period.step_change;
                height_offset += period.height_change;

                continue;
            }
        }

        loop {
            // First blow with gas
            match gas[gas_step] {
                Direction::Left => {
                    if position.0 != 0
                        && !check_intersection(&shape.points, position.0 - 1, position.1, &chamber)
                    {
                        position.0 -= 1;
                    }
                }
                Direction::Right => {
                    if !check_intersection(&shape.points, position.0 + 1, position.1, &chamber) {
                        position.0 += 1;
                    }
                }
            }
            gas_step = (gas_step + 1) % gas.len();

            // Now move down
            if position.1 != 0
                && !check_intersection(&shape.points, position.0, position.1 - 1, &chamber)
            {
                position.1 -= 1;
            } else {
                // We're resting on something now
                for point in shape.points.iter() {
                    assert!(!chamber[point.0 + position.0][point.1 + position.1]);
                    chamber[point.0 + position.0][point.1 + position.1] = true;

                    top_points[point.0 + position.0] =
                        top_points[point.0 + position.0].max(point.1 + position.1 + height_offset);
                    start.1 = start.1.max(point.1 + position.1 + 4);
                }

                let min = top_points.iter().min().unwrap().clone();
                for i in 0..top_points.len() {
                    top_points[i] -= min;
                }

                // Store this position
                let visit = CavePeriod {
                    gas_index: gas_step,
                    shape_index: i % shapes.len(),
                    top_points: top_points.clone(),
                };
                if let Some(height) = visited.get(&visit) {
                    println!(
                        "Period found {:?} {} {}",
                        height,
                        i,
                        (start.1 + height_offset)
                    );
                    period = Some(Period {
                        step_change: i - (*height).0,
                        height_change: (start.1 + height_offset) - (*height).1,
                    });
                }
                visited.insert(visit, (i, start.1 + height_offset));

                if check_line_covered(position.1, &chamber) {
                    for col in chamber.iter_mut() {
                        col.drain(0..position.1);
                    }
                    height_offset += position.1;
                    start.1 -= position.1;

                    for col in chamber.iter_mut() {
                        for _ in 0..position.1 {
                            col.push_back(false);
                        }
                    }
                }
                break;
            }
        }

        i += 1;
    }

    println!("{}", start.1 - 3 + height_offset);
}

#[derive(Debug)]
struct Period {
    step_change: usize,
    height_change: usize,
}

#[derive(Hash, PartialEq, Eq)]
struct CavePeriod {
    gas_index: usize,
    shape_index: usize,
    top_points: [usize; 7],
}

fn check_intersection(
    points: &Vec<(usize, usize)>,
    x: usize,
    y: usize,
    grid: &[VecDeque<bool>; 7],
) -> bool {
    points
        .iter()
        .any(|p| p.0 + x >= 7 || grid[p.0 + x][p.1 + y])
}

fn check_line_covered(y: usize, grid: &[VecDeque<bool>; 7]) -> bool {
    (0..7).map(|x| grid[x][y]).all(|b| b)
}

#[derive(Debug, Clone)]
struct Shape {
    points: Vec<(usize, usize)>,
}

enum Direction {
    Left,
    Right,
}

#[allow(dead_code)]
fn day_sixteen() {
    let data = read_file_to_string(Path::new("data/16.txt"));

    let graph = Graph {
        nodes: HashMap::from_iter(data.lines().map(|l| {
            let mut parts = l.split("=");
            let str = String::from_iter(parts.next().unwrap().chars().skip(6).take(2));
            let bytes = str.as_bytes();
            let label = ((bytes[0] as u32) << 8) + bytes[1] as u32;
            println!("{}", label);
            let mut parts = parts.next().unwrap().split("; ");
            let flow = str::parse(parts.next().unwrap()).unwrap();
            let edges = String::from_iter(parts.next().unwrap().chars().skip(23))
                .split(", ")
                .map(|s| ((s.as_bytes()[0] as u32) << 8) + s.as_bytes()[1] as u32)
                .collect_vec();

            (
                label,
                Node {
                    edges,
                    base_score: flow,
                },
            )
        })),
    };

    let mut visited = HashSet::new();
    let mut paths = VecDeque::new();

    let preopened = graph
        .nodes
        .iter()
        .filter(|(_, n)| n.base_score == 0)
        .map(|(l, _)| *l);

    paths.push_back(GraphPath {
        position: (("AA".as_bytes()[0] as u32) << 8) + "AA".as_bytes()[1] as u32,
        elephant_position: (("AA".as_bytes()[0] as u32) << 8) + "AA".as_bytes()[1] as u32,
        opened: HashSet::from_iter(preopened),
        score: 0,
        time_step: 26,
    });

    let mut best_score = 0;
    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        if path.time_step == 0 || path.opened.len() == graph.nodes.len() {
            if path.time_step != 0 && path.score > best_score {
                println!("{} {}", path.time_step, path.score);
            }
            best_score = best_score.max(path.score);
            continue;
        }

        let mut pos = vec![path.position, path.elephant_position];
        pos.sort();
        let visit = Visited {
            position: pos,
            time_step: path.time_step,
            opened: hashset_to_vec(&path.opened),
        };
        if visited.contains(&visit) {
            continue;
        }

        visited.insert(visit);
        if visited.len() % 10000 == 0 {
            println!("{}", visited.len());
        }

        if !path.opened.contains(&path.position)
            && !path.opened.contains(&path.elephant_position)
            && path.elephant_position != path.position
        {
            let mut new_opened = path.opened.clone();
            new_opened.insert(path.position);
            new_opened.insert(path.elephant_position);
            let score = (graph.nodes.get(&path.position).unwrap().base_score
                * (path.time_step - 1))
                + (graph.nodes.get(&path.elephant_position).unwrap().base_score
                    * (path.time_step - 1));
            paths.push_back(GraphPath {
                position: path.position,
                elephant_position: path.elephant_position,
                opened: new_opened,
                score: path.score + score,
                time_step: path.time_step - 1,
            });

            continue;
        }

        if !path.opened.contains(&path.position) {
            for elephant_edge in graph
                .nodes
                .get(&path.elephant_position)
                .unwrap()
                .edges
                .iter()
            {
                let mut new_opened = path.opened.clone();
                new_opened.insert(path.position.clone());
                paths.push_back(GraphPath {
                    position: path.position,
                    elephant_position: *elephant_edge,
                    opened: new_opened,
                    score: path.score
                        + (graph.nodes.get(&path.position).unwrap().base_score
                            * (path.time_step - 1)),
                    time_step: path.time_step - 1,
                });
            }

            continue;
        }

        if !path.opened.contains(&path.elephant_position) {
            for edge in graph.nodes.get(&path.position).unwrap().edges.iter() {
                let mut new_opened = path.opened.clone();
                new_opened.insert(path.elephant_position);
                paths.push_back(GraphPath {
                    position: *edge,
                    elephant_position: path.elephant_position,
                    opened: new_opened,
                    score: path.score
                        + (graph.nodes.get(&path.elephant_position).unwrap().base_score
                            * (path.time_step - 1)),
                    time_step: path.time_step - 1,
                });
            }

            continue;
        }

        for edge in graph.nodes.get(&path.position).unwrap().edges.iter() {
            for elephant_edge in graph
                .nodes
                .get(&path.elephant_position)
                .unwrap()
                .edges
                .iter()
            {
                paths.push_back(GraphPath {
                    position: *edge,
                    elephant_position: *elephant_edge,
                    opened: path.opened.clone(),
                    score: path.score,
                    time_step: path.time_step - 1,
                })
            }
        }
    }

    println!("{}", best_score);
}

fn hashset_to_vec(set: &HashSet<u32>) -> Vec<u32> {
    let mut out = vec![];
    for item in set {
        out.push(item.clone());
    }
    out.sort();

    out
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<u32, Node>,
}

#[derive(Debug)]
struct Node {
    edges: Vec<u32>,
    base_score: i64,
}

#[derive(Debug)]
struct GraphPath {
    position: u32,
    elephant_position: u32,
    opened: HashSet<u32>,
    score: i64,
    time_step: i64,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Visited {
    position: Vec<u32>,
    time_step: i64,
    opened: Vec<u32>,
}

#[allow(dead_code)]
fn day_fifteen() {
    let outer = 4000000;
    let data = read_file_to_string(Path::new("data/15.txt"));

    let sensors = data
        .lines()
        .map(|l| {
            let parts = l.split(':').collect_vec();
            let sensor: String = parts[0].chars().skip(10).collect();
            let (sx, sy): (i64, i64) = sensor
                .split(", ")
                .map(|s| s.split('=').skip(1).next().unwrap())
                .map(|s| str::parse(s).unwrap())
                .collect_tuple()
                .unwrap();

            let beacon: String = parts[1].chars().skip(21).collect();
            let (bx, by): (i64, i64) = beacon
                .split(", ")
                .map(|s| s.split('=').skip(1).next().unwrap())
                .map(|s| str::parse(s).unwrap())
                .collect_tuple()
                .unwrap();

            Sensor {
                x: sx,
                y: sy,
                beacon_x: bx,
                beacon_y: by,
                distance: ((sx - bx).abs() + (sy - by).abs()) as u64,
            }
        })
        .collect_vec();

    check_square(&sensors, 0, outer, 0, outer);
}

fn check_square(sensors: &Vec<Sensor>, x1: i64, x2: i64, y1: i64, y2: i64) -> (i64, i64) {
    if x2 - x1 <= 1 {
        if sensors.iter().all(|s| s.can_contain_hidden(x1, x1, y1, y1)) {
            println!("ANS: {x1} {x1} {y1} {y1}");
        }
        if sensors.iter().all(|s| s.can_contain_hidden(x1, x1, y2, y2)) {
            println!("ANS: {x1} {x1} {y2} {y2}");
        }
        return (0, 0);
    }

    let new_squares = [
        (x1, (x2 + x1) / 2, y1, (y2 + y1) / 2),
        ((x2 + x1) / 2, x2, y1, (y2 + y1) / 2),
        (x1, (x2 + x1) / 2, (y2 + y1) / 2, y2),
        ((x2 + x1) / 2, x2, (y2 + y1) / 2, y2),
    ];

    for (x1, x2, y1, y2) in new_squares {
        if sensors.iter().all(|s| s.can_contain_hidden(x1, x2, y1, y2)) {
            check_square(sensors, x1, x2, y1, y2);
        }
    }

    (0, 0)
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64,
    distance: u64,
}

impl Sensor {
    fn can_contain_hidden(&self, x1: i64, x2: i64, y1: i64, y2: i64) -> bool {
        (self.x - x1).abs() + (self.y - y1).abs() > self.distance as i64
            || (self.x - x2).abs() + (self.y - y1).abs() > self.distance as i64
            || (self.x - x1).abs() + (self.y - y2).abs() > self.distance as i64
            || (self.x - x2).abs() + (self.y - y2).abs() > self.distance as i64
    }
}

#[allow(dead_code)]
fn day_fourteen() {
    let data = read_file_to_string(Path::new("data/14.txt"));

    let mut grid = [[false; 400]; 400];
    let mut floor = 2;
    for line in data.lines() {
        let mut coords = line.split(" -> ");
        let mut current = coords
            .next()
            .unwrap()
            .split(",")
            .map(|s| str::parse::<i64>(s).unwrap())
            .collect_vec();
        for coord in coords {
            let next = coord
                .split(",")
                .map(|s| str::parse::<i64>(s).unwrap())
                .collect_vec();
            let change = if next[0] > current[0] {
                (1, 0)
            } else if next[0] < current[0] {
                (-1, 0)
            } else if next[1] > current[1] {
                (0, 1)
            } else {
                (0, -1)
            };

            while current[0] != next[0] || current[1] != next[1] {
                floor = floor.max(current[1] as usize + 2);
                grid[current[0] as usize - 300][current[1] as usize] = true;

                current[0] += change.0;
                current[1] += change.1;
            }
            floor = floor.max(current[1] as usize + 2);
            grid[current[0] as usize - 300][current[1] as usize] = true;
        }
    }
    for i in 0..grid.len() {
        grid[i][floor] = true;
    }

    println!("{}", floor);

    let mut total = 0;
    let mut fallen = false;
    while !fallen {
        let mut current = (200, 0);
        loop {
            if !grid[current.0][current.1 + 1] {
                current.1 += 1;
            } else if !grid[current.0 - 1][current.1 + 1] {
                current.0 -= 1;
                current.1 += 1;
            } else if !grid[current.0 + 1][current.1 + 1] {
                current.0 += 1;
                current.1 += 1;
            } else {
                if grid[current.0][current.1] {
                    fallen = true;
                    break;
                }
                grid[current.0][current.1] = true;
                total += 1;
                break;
            }
        }
    }

    println!("{total}");
}

#[allow(dead_code)]
fn day_thirteen() {
    let data = read_file_to_string(Path::new("data/13.txt"));
    let mut score = 0;

    for (i, packet) in data.split("\n\n").enumerate() {
        let pair: Vec<SignalItem> = packet.split("\n").map(|s| SignalItem::new(s)).collect();

        if let Some(false) = pair[0].compare(&pair[1]) {
        } else {
            score += i + 1;
        }
    }

    let list = data
        .split("\n")
        .filter(|l| *l != "")
        .map(|s| SignalItem::new(s))
        .sorted_by(|one, other| one.cmp(other))
        .collect_vec();

    println!("{}", score);
    println!(
        "{:?}",
        list.binary_search(&SignalItem::List(vec![SignalItem::List(vec![
            SignalItem::Number(6)
        ])]))
    );
    println!(
        "{:?}",
        list.binary_search(&SignalItem::List(vec![SignalItem::List(vec![
            SignalItem::Number(2)
        ])]))
    );
}

#[derive(Debug, PartialEq, Eq)]
enum SignalItem {
    Number(i64),
    List(Vec<SignalItem>),
}

impl SignalItem {
    fn new(raw: &str) -> Self {
        let mut lists = VecDeque::new();
        let mut digits = "".to_string();

        for char in raw.chars() {
            match char {
                '[' => {
                    lists.push_front(vec![]);
                }
                ']' => {
                    if digits != "" {
                        let number = str::parse(&digits).unwrap();
                        digits = "".to_string();
                        lists.get_mut(0).unwrap().push(SignalItem::Number(number));
                    }

                    if lists.len() != 1 {
                        let finished = lists.pop_front().unwrap();
                        lists.get_mut(0).unwrap().push(SignalItem::List(finished));
                    } else {
                        return SignalItem::List(lists.pop_front().unwrap());
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    digits.push(char);
                }
                ',' => {
                    if digits != "" {
                        let number = str::parse(&digits).unwrap();
                        digits = "".to_string();
                        lists.get_mut(0).unwrap().push(SignalItem::Number(number));
                    }
                }
                _ => panic!("Char was {}", char),
            }
        }

        panic!();
    }

    fn compare(&self, other: &Self) -> Option<bool> {
        match self {
            SignalItem::Number(n) => match other {
                SignalItem::Number(on) => {
                    if n == on {
                        None
                    } else {
                        Some(n < on)
                    }
                }
                SignalItem::List(_) => {
                    SignalItem::List(vec![SignalItem::Number(*n)]).compare(other)
                }
            },
            SignalItem::List(l) => match other {
                SignalItem::Number(n) => {
                    self.compare(&SignalItem::List(vec![SignalItem::Number(*n)]))
                }
                SignalItem::List(ol) => {
                    for i in 0..l.len() {
                        if i >= ol.len() {
                            return Some(false);
                        }

                        if let Some(res) = l[i].compare(&ol[i]) {
                            return Some(res);
                        }
                    }

                    if l.len() != ol.len() {
                        return Some(true);
                    }

                    return None;
                }
            },
        }
    }
}

impl PartialOrd for SignalItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SignalItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.compare(other) {
            Some(res) => {
                if res {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => Ordering::Equal,
        }
    }
}

#[allow(dead_code)]
fn day_twelve() {
    let data = read_file_to_string(Path::new("data/12.txt"));
    let goal = 'z' as u32;
    let mut starts = vec![];

    let mut grid = vec![];
    for (i, line) in data.lines().enumerate() {
        grid.push(vec![]);
        for (j, char) in line.chars().enumerate() {
            if char == 'E' {
                grid[i].push(goal);
            } else if char == 'S' || char == 'a' {
                grid[i].push('a' as u32);
                starts.push((j, i));
            } else {
                grid[i].push(char as u32);
            }
        }
    }

    let mut score = u32::MAX;

    for start in starts {
        let mut visited = HashSet::new();
        let mut paths = VecDeque::new();
        paths.push_back(GridPath {
            steps: 0,
            position: start,
        });

        while paths.len() != 0 {
            let path = paths.pop_front().unwrap();
            if grid[path.position.1][path.position.0] == goal {
                score = score.min(path.steps + 1);
                break;
            }

            if visited.contains(&path.position) {
                continue;
            }

            visited.insert(path.position);

            // Generate new moves
            let mut moves = vec![];
            if path.position.0 != 0 {
                moves.push((path.position.0 - 1, path.position.1));
            }
            if path.position.1 != 0 {
                moves.push((path.position.0, path.position.1 - 1));
            }
            if path.position.0 < grid[path.position.1].len() - 1 {
                moves.push((path.position.0 + 1, path.position.1));
            }
            if path.position.1 < grid.len() - 1 {
                moves.push((path.position.0, path.position.1 + 1));
            }

            moves = moves
                .into_iter()
                .filter(|m| grid[path.position.1][path.position.0] + 1 >= grid[m.1][m.0])
                .collect();
            for m in moves {
                paths.push_back(GridPath {
                    steps: path.steps + 1,
                    position: m,
                });
            }
        }
    }

    println!("{}", score);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct GridPath {
    steps: u32,
    position: (usize, usize),
}

#[allow(dead_code)]
fn day_eleven() {
    // let mut monkeys = vec![Monkey {
    //     items: VecDeque::from_iter(vec![79, 98]),
    //     operation: |old| {
    //         old * 19
    //     },
    //     test: |old| {
    //         if old % 23 == 0 {
    //             2
    //         } else {
    //             3
    //         }
    //     },
    //     inspected: 0,
    // },
    // Monkey {
    //     items: VecDeque::from_iter(vec![54, 65, 75, 74]),
    //     operation: |old| {
    //         old + 6
    //     },
    //     test: |old| {
    //         if old % 19 == 0 {
    //             2
    //         } else {
    //             0
    //         }
    //     },
    //     inspected: 0,
    // },
    // Monkey {
    //     items: VecDeque::from_iter(vec![79, 60, 97]),
    //     operation: |old| {
    //         old * old
    //     },
    //     test: |old| {
    //         if old % 13 == 0 {
    //             1
    //         } else {
    //             3
    //         }
    //     },
    //     inspected: 0,
    // },
    // Monkey {
    //     items: VecDeque::from_iter(vec![74]),
    //     operation: |old| {
    //         old + 3
    //     },
    //     test: |old| {
    //         if old % 17 == 0 {
    //             0
    //         } else {
    //             1
    //         }
    //     },
    //     inspected: 0,
    // },
    // ];

    let mut monkeys = vec![
        Monkey {
            items: VecDeque::from_iter(vec![91, 66]),
            operation: |old| old * 13,
            test: |old| {
                if old % 19 == 0 {
                    6
                } else {
                    2
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![78, 97, 59]),
            operation: |old| old + 7,
            test: |old| {
                if old % 5 == 0 {
                    0
                } else {
                    3
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![57, 59, 97, 84, 72, 83, 56, 76]),
            operation: |old| old + 6,
            test: |old| {
                if old % 11 == 0 {
                    5
                } else {
                    7
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![81, 78, 70, 58, 84]),
            operation: |old| old + 5,
            test: |old| {
                if old % 17 == 0 {
                    6
                } else {
                    0
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![60]),
            operation: |old| old + 8,
            test: |old| {
                if old % 7 == 0 {
                    1
                } else {
                    3
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![57, 69, 63, 75, 62, 77, 72]),
            operation: |old| old * 5,
            test: |old| {
                if old % 13 == 0 {
                    7
                } else {
                    4
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![73, 66, 86, 79, 98, 87]),
            operation: |old| old * old,
            test: |old| {
                if old % 3 == 0 {
                    5
                } else {
                    2
                }
            },
            inspected: 0,
        },
        Monkey {
            items: VecDeque::from_iter(vec![95, 89, 63, 67]),
            operation: |old| old + 2,
            test: |old| {
                if old % 2 == 0 {
                    1
                } else {
                    4
                }
            },
            inspected: 0,
        },
    ];

    let product = 19 * 5 * 11 * 17 * 7 * 13 * 3 * 2;
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                monkeys[i].inspected += 1;
                let new = (monkeys[i].operation)(monkeys[i].items[j]) % product;
                let new_monkey = (monkeys[i].test)(new);
                monkeys[new_monkey].items.push_back(new);
            }
            monkeys[i].items.clear()
        }

        for i in 0..monkeys.len() {
            println!("{} - {:?}", i, monkeys[i].inspected);
        }
        println!();
    }

    monkeys.sort();
    println!("{:?}", monkeys);
    println!(
        "{:?}",
        monkeys[monkeys.len() - 1].inspected * monkeys[monkeys.len() - 2].inspected
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<i64>,
    operation: fn(i64) -> i64,
    test: fn(i64) -> usize,
    inspected: usize,
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.inspected > other.inspected {
            Ordering::Greater
        } else if self.inspected == other.inspected {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

#[allow(dead_code)]
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
            score += current * cycle;
        }
        if ((cycle - 1) % 40).abs_diff(current) <= 1 {
            screen[(cycle - 1) as usize] = true
        }

        match op[0] {
            "noop" => {}
            "addx" => {
                cycle += 1;

                if ((cycle - 1) % 40).abs_diff(current) <= 1 {
                    screen[(cycle - 1) as usize] = true
                }
                if (cycle - 20) % 40 == 0 {
                    score += current * cycle;
                }
                current += str::parse::<isize>(op[1]).unwrap();
            }
            _ => panic!(),
        };
    }

    println!("{}", score);
    for i in 1..screen.len() + 1 {
        print!("{}", if screen[i - 1] { "#" } else { "." });

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
