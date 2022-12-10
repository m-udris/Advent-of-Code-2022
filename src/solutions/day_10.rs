use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn update_signal_strength(
    signal_strength: &mut isize,
    reg_x_value: isize,
    cycle_count: isize,
    cycle_lap_size: isize,
    cycle_lap_offset: isize,
    operation_cycle_count: isize
) {
    let cycle_lap_position = (cycle_count + cycle_lap_offset) % cycle_lap_size;
    let cycle_lap_next_position = (cycle_count + cycle_lap_offset + operation_cycle_count) % cycle_lap_size;

    if cycle_lap_next_position < cycle_lap_position + operation_cycle_count {
        *signal_strength += (cycle_count + operation_cycle_count - cycle_lap_next_position) * reg_x_value;
    }
}

pub fn run_first() -> Result<(), Box<dyn std::error::Error>> {
    const NOOP_CYCLE_COUNT: isize = 1;
    const ADDX_CYCLE_COUNT: isize = 2;
    const CYCLE_LAP_SIZE: isize = 40;
    const CYCLE_LAP_OFFSET: isize = 20;

    let mut signal_strength: isize = 0;
    let mut reg_x_value: isize = 1;
    let mut cycle_count: isize = 0;
    let file = File::open("./inputs/day_10/first.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {

        let unwrapped_line = line?;
        if let "noop" = unwrapped_line.trim() {
            update_signal_strength(
                &mut signal_strength,
                reg_x_value,
                cycle_count,
                CYCLE_LAP_SIZE,
                CYCLE_LAP_OFFSET,
                NOOP_CYCLE_COUNT
            );
            cycle_count += NOOP_CYCLE_COUNT;
        }
        if let ["addx", add_amount] = &unwrapped_line.split(' ').collect::<Vec<&str>>()[..] {
            update_signal_strength(
                &mut signal_strength,
                reg_x_value,
                cycle_count,
                CYCLE_LAP_SIZE,
                CYCLE_LAP_OFFSET,
                ADDX_CYCLE_COUNT
            );
            cycle_count += ADDX_CYCLE_COUNT;
            reg_x_value += add_amount.parse::<isize>()?;
        }
    }
    println!("{:?}", signal_strength);
    Ok(())
}


fn handle_crt(
    buffer: &mut Vec<char>,
    cycle_count: &mut usize,
    reg_x_value: isize,
    cycle_lap_size: usize
) {
    if reg_x_value - 1 <= (*cycle_count).try_into().unwrap()
        && reg_x_value + 1 >= (*cycle_count).try_into().unwrap()
    {
        buffer[*cycle_count] = '#';
    }

    *cycle_count += 1;

    if *cycle_count == cycle_lap_size {
        println!("{}", buffer.iter().collect::<String>());
        *buffer = vec!['.'; cycle_lap_size];
        *cycle_count %= cycle_lap_size;
    }
}


pub fn run_second() -> Result<(), Box<dyn std::error::Error>> {
    const ADDX_CYCLE_COUNT: isize = 2;
    const CYCLE_LAP_SIZE: usize = 40;

    let mut buffer: Vec<char> = vec!['.'; CYCLE_LAP_SIZE];

    let mut reg_x_value: isize = 1;
    let mut cycle_count: usize = 0;
    let file = File::open("./inputs/day_10/second.txt")
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {

        let unwrapped_line = line?;
        if let "noop" = unwrapped_line.trim() {
            handle_crt(
                &mut buffer,
                &mut cycle_count,
                reg_x_value,
                CYCLE_LAP_SIZE
            );
        }
        if let ["addx", add_amount] = &unwrapped_line.split(' ').collect::<Vec<&str>>()[..] {
            for _ in 0..ADDX_CYCLE_COUNT {
                handle_crt(
                    &mut buffer,
                    &mut cycle_count,
                    reg_x_value,
                    CYCLE_LAP_SIZE
                );
            }
            reg_x_value += add_amount.parse::<isize>()?;
        }
    }
    Ok(())
}