use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("day3_input.txt")?;
    let reader = BufReader::new(file);
    let mut total:u32 = 0;
    let line_count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line_bytes = line.as_bytes();
        if line_count < 2 {

        }
        let len = line.len();
        let len_half = len/2;
        let mut checked_chars: Vec<u8> = vec![];
        for c in 0..len_half {
            if !checked_chars.contains(&line_bytes[c]) {
                for i in len_half..len {
                    if line_bytes[c] == line_bytes[i] {
                        total += u32::from(get_value(line_bytes[c]));
                        break;
                    }
                }
                checked_chars.push(line_bytes[c]);
            }
            
        }
    }
    
    println!("Val: {}", get_value(b'L'));
    println!("Total: {}", total);
    Ok(())
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