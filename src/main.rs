use std::env;

mod day_1;
mod day_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        let (part_a, part_b) = match day.as_str() {
            "1" => day_1::day_1(),
            "2" => day_2::day_2(),
            _ => panic!(
                "Day {} has not yet been finished or added to the list of answers",
                day
            ),
        };

        println!("Answer for day {}:", day);
        println!("Part A: {}", part_a);
        println!("Part B: {}", part_b);
    } else {
        println!("pass a a number 1 - 25 to run the excercise");
    }
}
