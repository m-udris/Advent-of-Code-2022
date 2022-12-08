use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::zip;

fn get_matrix(input_filename: &str) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    let file = File::open(input_filename)
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in buf_reader.lines() {
        matrix.push(line?.trim().chars().map(|height| height.to_digit(10).unwrap()).collect());
    }

    Ok(matrix)
}

pub fn run_first() -> Result<(), Box<dyn std::error::Error>> {
    let matrix = get_matrix("./inputs/day_8/first.txt")?;

    let mut invisible_trees: usize = 0;
    let all_trees = matrix.iter().fold(0, |acc, v| acc + v.len());

    for (row_index, row) in matrix.iter().enumerate() {
        if row_index == 0 || row_index == matrix.len() - 1 {
            continue;
        }
        for (column_index, element) in row.iter().enumerate() {
            if column_index == 0 || column_index == row.len() - 1 {
                continue;
            }
            let column = matrix.iter().map(|row_vec| row_vec[column_index]).collect::<Vec<u32>>();

            let tallest_tree_left = row[0..column_index].iter().max().unwrap();
            let tallest_tree_right = row[column_index + 1..].iter().max().unwrap();
            let tallest_tree_up = column[0..row_index].iter().max().unwrap();
            let tallest_tree_down = column[row_index + 1..].iter().max().unwrap();

            let trees_around = [tallest_tree_left, tallest_tree_right, tallest_tree_up, tallest_tree_down];

            let shortest_tree_around = trees_around.iter().min().unwrap();

            if element <= shortest_tree_around {
                invisible_trees += 1;
            }
        }
    }
    let visible_trees = all_trees - invisible_trees;
    println!("{}", visible_trees);

    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>> {
    let matrix = get_matrix("./inputs/day_8/second.txt")?;

    let mut max_scenic_score = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        if row_index == 0 || row_index == matrix.len() - 1 {
            continue;
        }
        for (column_index, element) in row.iter().enumerate() {
            if column_index == 0 || column_index == row.len() - 1 {
                continue;
            }
            let column = matrix.iter().map(|row_vec| row_vec[column_index]).collect::<Vec<u32>>();

            let tallest_tree_left = row[..column_index].iter().rev().position(|height| height >= element).unwrap_or(row[..column_index].len() - 1) + 1;
            let tallest_tree_right = row[column_index + 1..].iter().position(|height| height >= element).unwrap_or(row[column_index + 1..].len() - 1) + 1;
            let tallest_tree_up = column[..row_index].iter().rev().position(|height| height >= element).unwrap_or(column[..row_index].len() - 1) + 1;
            let tallest_tree_down = column[row_index + 1..].iter().position(|height| height >= element).unwrap_or(column[row_index + 1..].len() - 1) + 1;

            let scenic_score = tallest_tree_left * tallest_tree_right * tallest_tree_up * tallest_tree_down;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;

            }
        }
    }
    println!("{}", max_scenic_score);

    Ok(())
}