use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day1_input.txt")?;
    let reader = BufReader::new(file);

    let mut current = 0;
    let mut calorie_list = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            calorie_list.push(current);
            current = 0;
        } else {
            let parsed = line.parse::<i32>().unwrap();
            current += parsed;
        }
    }

    calorie_list.sort();
    let length = calorie_list.len();
    let top3 = calorie_list[length-1] + calorie_list[length-2] + calorie_list[length-3];
    println!("Top 3 Calorie Total: {}", top3);
    Ok(())
}