use crate::utils;

pub fn day_1(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(report: &str) -> u16 {
    let depths = utils::input_to_numbers(report);

    depths.windows(2).fold(0, |depth, nums| match nums {
        [n1, n2] if n2 > n1 => depth + 1,
        _ => depth,
    })
}

fn part_2(report: &str) -> u16 {
    let depths = utils::input_to_numbers(report);

    depths.windows(4).fold(0, |depth, nums| match nums {
        [a, b, c, d] if b + c + d > a + b + c => depth + 1,
        _ => depth,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 5);
    }
}
