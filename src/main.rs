mod solutions;
use crate::solutions::{day_1, day_2, day_3, day_4, day_5, day_6, day_7};

fn main() {
    let problem = "7_1";

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
        _ => println!("No such solution found."),
    };

}
