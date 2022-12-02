mod solutions;
use crate::solutions::{day_1, day_2};

fn main() {
    let problem = "2_2";

    match problem {
        "1_1" => day_1::run_first().unwrap(),
        "1_2" => day_1::run_second().unwrap(),
        "2_1" => day_2::run_first().unwrap(),
        "2_2" => day_2::run_second().unwrap(),
        _ => println!("No such solution found."),
    };

}
