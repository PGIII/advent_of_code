use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
  let file = File::open("src/day5_input.txt")?;
  let reader = BufReader::new(file);
  pt1(reader);
  Ok(())
}

fn pt1(reader: BufReader<File>) {
  let stacks: [Vec<char>; 3];

  for line in reader.lines() {
    let line = line.unwrap();
    if line.starts_with(" 1 ") {
      //line is last of stacks
    } else if line.starts_with("move") {
      //move command
    } else {
      //handle stacking
      println!("{}\r\n{:?}", line, line.as_bytes());
    }
  }
}