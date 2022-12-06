use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use itertools::Itertools;


fn run_task(input: &str, window_size: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open(input)
    .expect("file not found!");
    let buf_reader = BufReader::new(file);

    if let Some(input_line) = buf_reader.lines().next() {
        let unwrapped_line = input_line?;
        let chars_vec = unwrapped_line.chars().collect::<Vec<char>>();
        let a = chars_vec.windows(window_size).enumerate().find(|(_index, window)| Vec::<char>::from(*window).into_iter().unique().collect::<Vec<char>>().len() == window_size);
        if let Some((index, _)) = a {
            return Ok(index + window_size);
        }
    }
    panic!();
}

pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    let result = run_task("./inputs/day_6/first.txt", 4);
    println!("{}", result?);
    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    let result = run_task("./inputs/day_6/second.txt", 14);
    println!("{}", result?);
    Ok(())
}