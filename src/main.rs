mod solutions;
use crate::solutions::day_1;

fn main() {
    let problem = "1_1";

    match problem {
        "1_1" => day_1::run_first().unwrap(),
        "1_2" => day_1::run_second().unwrap(),
        _ => println!("No such solution found."),
    };

}
