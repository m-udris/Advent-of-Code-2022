mod solutions;
use crate::solutions::*;

fn main() {
    let problem = "9_2";

    match problem {
        "1_1" => day_1::run_first().unwrap(),
        "1_2" => day_1::run_second().unwrap(),
        "2_1" => day_2::run_first().unwrap(),
        "2_2" => day_2::run_second().unwrap(),
        "3_1" => day_3::run_first().unwrap(),
        "3_2" => day_3::run_second().unwrap(),
        "4_1" => day_4::run_first().unwrap(),
        "4_2" => day_4::run_second().unwrap(),
        "5_1" => day_5::run_first().unwrap(),
        "5_2" => day_5::run_second().unwrap(),
        "6_1" => day_6::run_first().unwrap(),
        "6_2" => day_6::run_second().unwrap(),
        "7_1" => day_7::run_first().unwrap(),
        "7_2" => day_7::run_second().unwrap(),
        "8_1" => day_8::run_first().unwrap(),
        "8_2" => day_8::run_second().unwrap(),
        "9_1" => day_9::run_first().unwrap(),
        "9_2" => day_9::run_second().unwrap(),
        _ => println!("No such solution found."),
    };

}
