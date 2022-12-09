use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;


#[derive(Debug, Default, Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn get_distance_from(&self, position: &Position) -> Position {
        Position {
            x: position.x - self.x,
            y: position.y - self.y,
        }
    }
    pub fn move_x(&mut self, x: i32) {
        self.x += x;
    }
    pub fn move_y(&mut self, y: i32) {
        self.y += y;
    }
}



pub fn run_first() -> Result<(), Box<dyn std::error::Error>>  {
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut unique_tail_positions = HashSet::<Position>::new();
    unique_tail_positions.insert(tail.clone());

    let file = File::open("./inputs/day_9/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if let [direction, steps_string] = unwrapped_line.split(' ').collect::<Vec<&str>>()[..] {
            let steps_amount = steps_string.parse::<i32>()?;
            match direction {
                "U" => head.move_y(steps_amount),
                "D" => head.move_y(-steps_amount),
                "L" => head.move_x(-steps_amount),
                "R" => head.move_x(steps_amount),
                _ => unreachable!()
            };
            let mut distance_apart = head.get_distance_from(&tail);

            while distance_apart.x.abs() > 1 || distance_apart.y.abs() > 1 {
                tail.move_x(-distance_apart.x.signum());
                tail.move_y(-distance_apart.y.signum());
                unique_tail_positions.insert(tail.clone());
                distance_apart = head.get_distance_from(&tail);
            }
        }
    }
    println!("{:?}", unique_tail_positions.len());
    Ok(())
}

pub fn run_second() -> Result<(), Box<dyn std::error::Error>>  {
    const NODE_COUNT: usize = 10;
    let mut nodes: Vec<Position> = vec![Position::default(); NODE_COUNT];
    let mut unique_tail_positions = HashSet::<Position>::new();
    unique_tail_positions.insert(Position::default());

    let file = File::open("./inputs/day_9/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let unwrapped_line = line?;
        if let [direction, steps_string] = unwrapped_line.split(' ').collect::<Vec<&str>>()[..] {
            let steps_amount = steps_string.parse::<i32>()?;
            for _ in 0..steps_amount {
                let root: &mut Position = nodes.get_mut(0)
                    .expect("Uh oh, the head of the rope disappeared into the river!");

                match direction {
                    "U" => root.move_y(1),
                    "D" => root.move_y(-1),
                    "L" => root.move_x(-1),
                    "R" => root.move_x(1),
                    _ => unreachable!()
                };

                for i in 0..(NODE_COUNT - 1) {
                    let head = nodes.get(i).unwrap().clone();
                    let tail = nodes.get_mut(i + 1).unwrap();

                    let mut distance_apart = head.get_distance_from(&tail);

                    while distance_apart.x.abs() > 1 || distance_apart.y.abs() > 1 {
                        tail.move_x(-distance_apart.x.signum());
                        tail.move_y(-distance_apart.y.signum());
                        if i == (NODE_COUNT - 2) {
                            unique_tail_positions.insert(tail.clone());
                        }
                        distance_apart = head.get_distance_from(&tail);
                    }
                }
            }
        }
    }
    println!("{:?}", unique_tail_positions.len());
    Ok(())
}