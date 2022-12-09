use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/day3_input.txt")?;
    let reader = BufReader::new(file);
    let mut total:u32 = 0;
    let mut current_line = 0;
    let mut total_lines = 0;
    let mut group: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
    let mut groups: Vec<[String; 3]> = vec![];
    for line in reader.lines() {
        group[current_line] = line.unwrap();
        current_line += 1;
        total_lines += 1;
        if current_line == 3 {
            let badge = find_badge(&group);
            total += get_value(badge as u8) as u32;
            groups.push(group.clone());
            current_line = 0;
            //println!("Badge Is {}", badge);
        }
    }

    assert_eq!(total_lines/3, groups.len());
    println!("Total: {}", total);
    Ok(())
}

fn find_badge(group: &[String; 3]) -> char {
    let mut shortest = 0;
    let mut shortest_string_index = 4;
    for i in 0..3 {
        if shortest_string_index == 4 {
            shortest = group[i].len();
            shortest_string_index = i;
        } else {
            if group[i].len() < shortest {
                shortest = group[i].len();
                shortest_string_index = i;
            }
        }
    }
    
    for c in 0..shortest {
        let character = group[shortest_string_index].as_bytes()[c] as char;
        //check if this char is in all 
        match shortest_string_index {
            0 => {
                //check 1 and 2
                if group[1].contains(character) && group[2].contains(character){
                    return character;
                }
            },
            1 => {
                //check 0 and 2
                if group[0].contains(character) && group[2].contains(character){
                    return character;
                }
            },
            2 => {
                //check 0 and 1
                if group[1].contains(character) && group[0].contains(character){
                    return character;
                }
            },
            _ => {}
        }
    }
    return '\0';
}

fn get_value(input: u8) -> u8 {
    if input <= b'z' && input >= b'a' {
        return lower_case_calc(input);
    } else {
        return upper_case_calc(input);
    }
}

fn lower_case_calc(input:u8) -> u8 {
    let from_z = b'z' - input;
    let result = 26 - from_z;
    return result;
}

fn upper_case_calc(input:u8) -> u8 {
    let from_z = b'Z' - input;
    let result = 52 - from_z;
    return result;
}