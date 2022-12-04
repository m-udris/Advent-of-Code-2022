use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    let mut answer: usize = 0;

    let file = File::open("./inputs/day_4/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        let elf_ranges = unwrapped_line.split(',').collect::<Vec<&str>>();
        let first_elf_range = elf_ranges[0].split('-').collect::<Vec<&str>>();
        let second_elf_range = elf_ranges[1].split('-').collect::<Vec<&str>>();

        let (first_elf_start, first_elf_end) = (first_elf_range[0].parse::<usize>()?, first_elf_range[1].parse::<usize>()?);
        let (second_elf_start, second_elf_end) = (second_elf_range[0].parse::<usize>()?, second_elf_range[1].parse::<usize>()?);

        if first_elf_start <= second_elf_start && first_elf_end >= second_elf_end
            || second_elf_start <= first_elf_start && second_elf_end >= first_elf_end {
            answer += 1;
        }
    }

    println!("{}", answer);

    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    let mut answer: usize = 0;

    let file = File::open("./inputs/day_4/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        let elf_ranges = unwrapped_line.split(',').collect::<Vec<&str>>();
        let first_elf_range = elf_ranges[0].split('-').collect::<Vec<&str>>();
        let second_elf_range = elf_ranges[1].split('-').collect::<Vec<&str>>();

        let (first_elf_start, first_elf_end) = (first_elf_range[0].parse::<usize>()?, first_elf_range[1].parse::<usize>()?);
        let (second_elf_start, second_elf_end) = (second_elf_range[0].parse::<usize>()?, second_elf_range[1].parse::<usize>()?);

        if !(first_elf_end < second_elf_start || second_elf_end < first_elf_start) {
            answer += 1;
        }
    }

    println!("{}", answer);

    Ok(())
}