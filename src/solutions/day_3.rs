use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn get_char_cost(input: char) -> usize {
    if input.is_uppercase() {
        return 26 + input as usize - 64;
    }
    else if input.is_lowercase() {
        return input as usize - 96;
    }
    return 0;
}


pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    fn string_to_sorted_chars(input: &str) -> Vec<char> {
        let mut input_chars: Vec<char> = input.chars().collect();
        input_chars.sort_by(|a, b| b.cmp(a));
        return input_chars;
    }

    let mut answer: usize = 0;

    let file = File::open("./inputs/day_3/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if unwrapped_line == "" {
            continue;
        }
        let (first_compartment, second_compartment) = unwrapped_line.split_at(unwrapped_line.len() / 2 as usize);

        let first_compartment_sorted_chars = string_to_sorted_chars(&first_compartment);
        let second_compartment_sorted_chars = string_to_sorted_chars(&second_compartment);

        let mut result = first_compartment_sorted_chars
            .iter()
            .filter(|c| second_compartment_sorted_chars.contains(c))
            .collect::<Vec<_>>();
        result.dedup();

        let shared_char = *result[0];
        answer += get_char_cost(shared_char);
    }
    println!("{}", answer);
    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    let mut answer: usize = 0;
    let mut shared_chars: Vec<char> = vec![];

    let file = File::open("./inputs/day_3/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for (index, line) in buf_reader.lines().enumerate() {
        let unwrapped_line = line?;
        let line_char_vector = unwrapped_line.chars().collect::<Vec<_>>();
        match index % 3 {
            0 => {
                shared_chars = line_char_vector;
                shared_chars.dedup();
            },
            1 => {
                shared_chars
                    .retain(|c| line_char_vector.contains(c));
            },
            2 => {
                shared_chars
                    .retain(|c| line_char_vector.contains(c));
                let shared_char = shared_chars[0];
                answer += get_char_cost(shared_char);
            },
            _ => panic!("Uh oh!"),
        };
    }
    println!("{}", answer);
    Ok(())
}