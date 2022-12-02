use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    let possible_opponents_hands = vec!["A", "B", "C"];
    let possible_answers = vec!["X", "Y", "Z"];
    let mut current_score: usize = 0;

    let file = File::open("./inputs/day_2/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        if let [opponents_hand, answer] = &unwrapped_line.split_whitespace().collect::<Vec<&str>>()[..] {
            let opponents_hand_position = possible_opponents_hands.iter().position(|&r| &r == opponents_hand).unwrap();
            let answer_position = possible_answers.iter().position(|&r| &r == answer).unwrap();
            let choice_score = answer_position + 1;
            let result_score = (3 + ((3 + answer_position - opponents_hand_position) % 3) * 3 ) % 9;
            current_score += choice_score + result_score;
            println!("{} {} {} {}", opponents_hand, answer, choice_score, result_score);
        }

    }
    println!("{}", current_score);
    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    let possible_opponents_hands = vec!["A", "B", "C"];
    let possible_answers = vec!["X", "Y", "Z"];
    let mut current_score: usize = 0;

    let file = File::open("./inputs/day_2/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        if let [opponents_hand, answer] = &unwrapped_line.split_whitespace().collect::<Vec<&str>>()[..] {
            let opponents_hand_position = possible_opponents_hands.iter().position(|&r| &r == opponents_hand).unwrap();
            let answer_position = possible_answers.iter().position(|&r| &r == answer).unwrap();
            let choice_score = (2 + answer_position + opponents_hand_position) % 3 + 1;
            let result_score = answer_position * 3;
            current_score += choice_score + result_score;
            println!("{} {} {} {}", opponents_hand, answer, choice_score, result_score);
        }

    }
    println!("{}", current_score);
    Ok(())
}