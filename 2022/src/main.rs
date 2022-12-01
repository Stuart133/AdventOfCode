use std::{fs::File, io::Read, path::Path};

fn main() {
    day_one();
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
