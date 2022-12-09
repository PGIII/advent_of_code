use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct ElfJobs {
  high: usize,
  low: usize,
}

fn main() -> io::Result<()> {
  let file = File::open("src/day4_input.txt")?;
  let reader = BufReader::new(file);
  //pt1(reader);
  pt2(reader);
  Ok(())
}

fn pt1(reader: BufReader<File>) {
  let mut fully_contained_count = 0;
  for line in reader.lines() {
    let mut elf1 = ElfJobs {high: 0, low: 0};
    let mut elf2 = ElfJobs {high: 0, low: 0};

    //split each elf based on,
    let line_str = line.unwrap();
    let elf_strings = line_str.split(",").collect::<Vec<&str>>();
    let elf1_split = elf_strings[0].split("-").collect::<Vec<&str>>();
    let elf2_split = elf_strings[1].split("-").collect::<Vec<&str>>();
    elf1.low = elf1_split[0].parse::<usize>().unwrap();
    elf1.high = elf1_split[1].parse::<usize>().unwrap();
    elf2.low = elf2_split[0].parse::<usize>().unwrap();
    elf2.high = elf2_split[1].parse::<usize>().unwrap();

    if elf1.high <= elf2.high && elf1.low >= elf2.low {
      fully_contained_count += 1;
    } else if elf2.high <= elf1.high && elf2.low >= elf1.low {
      fully_contained_count += 1;
    }
  }
  
  println!("Fully Contained {}", fully_contained_count);
}

fn pt2(reader: BufReader<File>) {
  let mut overlap = 0;
  for line in reader.lines() {
    let mut elf1 = ElfJobs {high: 0, low: 0};
    let mut elf2 = ElfJobs {high: 0, low: 0};

    //split each elf based on,
    let line_str = line.unwrap();
    let elf_strings = line_str.split(",").collect::<Vec<&str>>();
    let elf1_split = elf_strings[0].split("-").collect::<Vec<&str>>();
    let elf2_split = elf_strings[1].split("-").collect::<Vec<&str>>();
    elf1.low = elf1_split[0].parse::<usize>().unwrap();
    elf1.high = elf1_split[1].parse::<usize>().unwrap();
    elf2.low = elf2_split[0].parse::<usize>().unwrap();
    elf2.high = elf2_split[1].parse::<usize>().unwrap();

    if elf1.high <= elf2.high && elf1.high >= elf2.low {
      overlap  += 1;
    } else if elf1.low >= elf2.low && elf1.low <= elf2.high {
      overlap += 1;
    } else if elf2.low >= elf1.low && elf2.low <= elf1.high {
      overlap += 1;
    } else if elf2.low >= elf1.low && elf2.low <= elf1.high {
      overlap += 1;
    }
  }
  
  println!("Partial Contained {}", overlap);
}