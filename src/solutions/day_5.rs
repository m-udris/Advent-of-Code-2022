use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::VecDeque;


fn pack_crates_from_string(crate_piles: &mut Vec<VecDeque<char>>, line: &String) -> bool {
    for (index, character) in line.chars().enumerate() {
        if index == 0 || (index - 1) % 4 != 0 {
            continue;
        }
        let pile_index = (index - 1) / 4 as usize;
        if crate_piles.len() <= pile_index {
            crate_piles.push(VecDeque::new());
        }
        if character.is_alphabetic() {
            crate_piles[pile_index].push_back(character);
        }
        if character.is_digit(10) {
            return true;
        }
    }
    false
}


pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    let mut crate_piles: Vec<VecDeque<char>> = vec![];
    let mut is_instruction_stage = false;


    let file = File::open("./inputs/day_5/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        if is_instruction_stage {
            let instruction_words = &unwrapped_line.split_whitespace().collect::<Vec<&str>>()[..];
            if let [_, amount, _, source, _, target] = instruction_words {
                let source_index = source.parse::<usize>()? - 1;
                let target_index = target.parse::<usize>()? - 1;
                for _ in 0..(amount.parse::<usize>()?) {
                    let crate_to_move = crate_piles[source_index].pop_front().unwrap_or('!');
                    crate_piles[target_index].push_front(crate_to_move);
                }
            }
        }
        else {
            is_instruction_stage = pack_crates_from_string(&mut crate_piles, &unwrapped_line);
        }
    }
    let mut top_pile_crates = String::new();
    for mut crate_pile in crate_piles {
        top_pile_crates.push(crate_pile.pop_front().unwrap_or('!'));
    }
    println!("{}", top_pile_crates);
    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    let mut crate_piles: Vec<VecDeque<char>> = vec![];
    let mut is_instruction_stage = false;


    let file = File::open("./inputs/day_5/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        if is_instruction_stage {
            let instruction_words = &unwrapped_line.split_whitespace().collect::<Vec<&str>>()[..];
            if let [_, amount, _, source, _, target] = instruction_words {
                let source_index = source.parse::<usize>()? - 1;
                let target_index = target.parse::<usize>()? - 1;

                let mut crates_to_move: VecDeque<char> = VecDeque::new();
                for _ in 0..(amount.parse::<usize>()?) {
                    crates_to_move.push_front(crate_piles[source_index].pop_front().unwrap_or('!'));
                }
                for crate_to_move in crates_to_move {
                    crate_piles[target_index].push_front(crate_to_move);
                }
            }
        }
        else {
            is_instruction_stage = pack_crates_from_string(&mut crate_piles, &unwrapped_line);
        }
    }
    let mut top_pile_crates = String::new();
    for mut crate_pile in crate_piles {
        top_pile_crates.push(crate_pile.pop_front().unwrap_or('!'));
    }
    println!("{}", top_pile_crates);
    Ok(())
}