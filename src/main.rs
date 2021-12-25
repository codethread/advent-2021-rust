use std::env;

use advent_of_code_2021_rust::run_day;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        let (part_a, part_b) = run_day(day);

        println!("Answer for day {}:", day);
        println!("Part A: {}", part_a);
        println!("Part B: {}", part_b);
    } else {
        println!("pass a a number 1 - 25 to run the excercise");
    }
}
