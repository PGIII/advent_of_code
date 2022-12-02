use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let rock_points = 1;
    let paper_points = 2;
    let scissor_points = 3;
    let lose_points = 0;
    let tie_points = 3;
    let win_points = 6;
    //abc = rock paper scissors
    //xyz = lose tie win
    //lose 0pt tie 3pt win 6pts

    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let moves: Vec<&str>;
        moves = line.split_ascii_whitespace().collect();

        match moves[0] {
            "A" => { //rock
                match moves[1] {
                    "X" => { // lose
                        score += scissor_points;
                        score += lose_points;
                    }
                    "Y" => {
                        score += rock_points;
                        score += tie_points;
                    }
                    "Z" => {
                        score += paper_points;
                        score += win_points;
                    }
                    _ => {panic!("Invalid user move")}
                }
            }
            "B" => {//paper
                match moves[1] {
                    "X" => { // lose
                        score += rock_points;
                        score += lose_points;
                    }
                    "Y" => {
                        score += paper_points;
                        score += tie_points;
                    }
                    "Z" => {
                        score += scissor_points;
                        score += win_points;
                    }
                    _ => {panic!("Invalid user move")}
                }
            }
            "C" => {//scissors
                match moves[1] {
                    "X" => { // lose
                        score += paper_points;
                        score += lose_points;
                    }
                    "Y" => {
                        score += scissor_points;
                        score += tie_points;
                    }
                    "Z" => {
                        score += rock_points;
                        score += win_points;
                    }
                    _ => {panic!("Invalid user move")}
                }
            }
            _ => {panic!("Invalid opponent move")}
        }
    }
    println!("Final Score {}", score);
    Ok(())
}