use crate::utils;

pub fn day_1(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(report: &str) -> u16 {
    let depths: Vec<u16> = utils::input_to_numbers(report);

    depths.windows(2).fold(0, |depth, ns| match ns {
        [n1, n2] => {
            if n2 > n1 {
                depth + 1
            } else {
                depth
            }
        }
        _ => unreachable!(),
    })
}

fn part_2(report: &str) -> u16 {
    let depths: Vec<u16> = utils::input_to_numbers(report);

    depths
        .windows(3)
        .map(|nums| nums.iter().sum::<u16>())
        .collect::<Vec<u16>>()
        .windows(2)
        .fold(0, |depth, ns| match ns {
            [n1, n2] => {
                if n2 > n1 {
                    depth + 1
                } else {
                    depth
                }
            }
            _ => unreachable!(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(part_1(input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(part_2(input), 5);
    }
}
