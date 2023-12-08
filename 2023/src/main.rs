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
            let nums: Vec<char> = l.chars().filter(|c| c.is_numeric()).collect();

            format!("{}{}", nums[0], nums[nums.len() - 1])
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("{count}");
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
