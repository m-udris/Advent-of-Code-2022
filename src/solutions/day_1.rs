use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    let mut current_max_calories: usize = 0;
    let mut current_calories: usize = 0;

    let file = File::open("./inputs/day_1/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            current_max_calories = std::cmp::max(current_calories, current_max_calories);
            current_calories = 0;
        }
        else {
            let calories = unwrapped_line.parse::<usize>()?;
            current_calories += calories;
        }
    }
    if current_calories != 0 {
        current_max_calories = std::cmp::max(current_calories, current_max_calories);
    }
    println!("{}", current_max_calories);

    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    fn update_top_carried_calories(top_carried_calories: &mut Vec<usize>, current_calories: usize) {
        top_carried_calories.push(current_calories);
        if top_carried_calories.len() > 3 {
            top_carried_calories.sort_by(|a, b| b.cmp(a));
            top_carried_calories.pop();
        }
    }

    let mut top_carried_calories = vec![];
    let mut current_calories: usize = 0;

    let file = File::open("./inputs/day_1/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            update_top_carried_calories(&mut top_carried_calories, current_calories);
            current_calories = 0;
        }
        else {
            let calories = unwrapped_line.parse::<usize>()?;
            current_calories += calories;
        }
    }
    if current_calories != 0 {
        update_top_carried_calories(&mut top_carried_calories, current_calories);
    }
    println!("{}", top_carried_calories.iter().sum::<usize>());

    Ok(())
}