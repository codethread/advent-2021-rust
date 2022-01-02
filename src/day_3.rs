pub fn day_3(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(diagnostic: &str) -> u32 {
    SubDiagnostics::from(diagnostic).get_power_consumption()
}

fn part_2(_: &str) -> u32 {
    0
}

#[derive(Debug)]
struct SubDiagnostics {
    pub bits: Vec<u16>,
    length: u16,
}

impl SubDiagnostics {
    fn new() -> Self {
        SubDiagnostics {
            bits: Vec::new(),
            length: 0,
        }
    }

    fn get_power_consumption(&self) -> u32 {
        let gamme_rate: String = self
            .bits
            .iter()
            .map(|sum| if sum > &(self.length / 2) { '1' } else { '0' })
            .collect();

        let epsilon_rate: String = gamme_rate
            .chars()
            .map(|c| if c == '1' { '0' } else { '1' })
            .collect();

        let gamme_rate = u32::from_str_radix(gamme_rate.as_str(), 2).unwrap();
        let epsilon_rate = u32::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

        gamme_rate * epsilon_rate
    }
}

impl From<&str> for SubDiagnostics {
    fn from(binary_data: &str) -> Self {
        let width = &binary_data.find('\n').unwrap() + 1;

        binary_data
            .char_indices()
            .fold(SubDiagnostics::new(), |mut vert, (i, c)| {
                match c {
                    '\n' => {
                        vert.length += 1;
                    }
                    '0' => {
                        if vert.bits.get_mut(i % width).is_none() {
                            vert.bits.insert(i % width, 0);
                        }
                    }
                    '1' => match vert.bits.get_mut(i % width) {
                        Some(n) => {
                            *n += 1;
                        }
                        None => {
                            vert.bits.insert(i % width, 1);
                        }
                    },
                    _ => unreachable!(),
                };
                vert
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 198);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}
