use std::fs::read_to_string;

mod day_1;
mod day_2;
mod utils;

pub fn run_day(day: &str) -> (String, String) {
    let input = read_to_string(format!("inputs/day-{}.txt", &day))
        .unwrap_or_else(|_| panic!("could not read input for day {}", &day))
        .trim()
        .to_owned();

    match day {
        "1" => day_1::day_1(&input),
        "2" => day_2::day_2(&input),
        _ => panic!(
            "Day {} has not yet been finished or added to the list of answers",
            day
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::run_day;

    #[test]
    fn test_run_day_1() {
        assert_eq!(run_day("1"), ("1681".to_string(), "1704".to_string()))
    }

    #[test]
    fn test_run_day_2() {
        assert_eq!(
            run_day("2"),
            ("1507611".to_string(), "1880593125".to_string())
        )
    }
}
