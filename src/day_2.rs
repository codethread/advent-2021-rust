pub fn day_2(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

struct Position {
    x: u32,
    y: u32,
}

impl Position {
    pub fn down(mut self, magnitude: u32) -> Self {
        self.y += magnitude;
        self
    }

    pub fn up(mut self, magnitude: u32) -> Self {
        self.y -= magnitude;
        self
    }

    pub fn forward(mut self, magnitude: u32) -> Self {
        self.x += magnitude;
        self
    }

    pub fn vector(&self) -> u32 {
        self.x * self.y
    }
}

fn part_1(directions: &str) -> u32 {
    directions
        .lines()
        .fold(Position { x: 0, y: 0 }, |pos, instruction| {
            let (direction, magnitude) = parse_instruction(instruction);

            match direction {
                "forward" => pos.forward(magnitude),
                "down" => pos.down(magnitude),
                "up" => pos.up(magnitude),
                _ => unreachable!(),
            }
        })
        .vector()
}

struct Sub {
    aim: u32,
    horizontal: u32,
    depth: u32,
}

impl Sub {
    pub fn new() -> Self {
        Sub {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn up(mut self, magnitude: u32) -> Self {
        self.aim -= magnitude;
        self
    }

    pub fn down(mut self, magnitude: u32) -> Self {
        self.aim += magnitude;
        self
    }

    pub fn forward(mut self, magnitude: u32) -> Self {
        self.horizontal += magnitude;
        self.depth += self.aim * magnitude;
        self
    }

    pub fn vector(&self) -> u32 {
        self.horizontal * self.depth
    }
}

fn part_2(directions: &str) -> u32 {
    directions
        .lines()
        .fold(Sub::new(), |sub, instruction| {
            let (direction, magnitude) = parse_instruction(instruction);

            match direction {
                "up" => sub.up(magnitude),
                "down" => sub.down(magnitude),
                "forward" => sub.forward(magnitude),
                _ => unreachable!(),
            }
        })
        .vector()
}

fn parse_instruction(instruction: &str) -> (&str, u32) {
    let (direction, magnitude) = instruction
        .split_once(" ")
        .unwrap_or_else(|| panic!("invalid direction: '{}'", &instruction));

    let magnitude: u32 = magnitude
        .parse()
        .unwrap_or_else(|_| panic!("could not parse '{}' to number", &magnitude));

    (direction, magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 150);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 900);
    }
}
