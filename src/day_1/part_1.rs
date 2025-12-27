use std::fmt::Display;

const DIAL_MAX: u8 = 99;
struct Instruction {
    direction: Direction,
    clicks: u32,
}
enum Direction {
    Left,
    Right,
}
struct Dial {
    password: u32,
    current: u8,
}
impl Dial {
    fn new() -> Self {
        Self {
            password: 0,
            current: 50,
        }
    }

    fn turn(&mut self, instruction: Instruction) {
        let max = DIAL_MAX as i8;
        let mut clicks_remaining = instruction.clicks;
        let mut temp_click: i8 = self.current as i8;

        while clicks_remaining > 0 {
            match instruction.direction {
                Direction::Left => {
                    temp_click -= 1;
                }
                Direction::Right => {
                    temp_click += 1;
                }
            };

            if temp_click > max {
                temp_click = 0;
            }

            if temp_click < 0 {
                temp_click += max + 1;
            }

            clicks_remaining -= 1;
        }

        if temp_click == 0 {
            self.password += 1;
        }

        self.current = temp_click as u8
    }
}

impl Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Current position: {}", self.current)
    }
}

fn part_1(input: &str) -> u32 {
    let input: Vec<_> = input.split('\n').collect();

    let instructions = input
        .iter()
        .map(|line| {
            let dir = match line.chars().next() {
                Some('L') => Direction::Left,
                Some('R') => Direction::Right,
                _ => unreachable!(),
            };

            let clicks = line[1..]
                .parse::<u32>()
                .expect("Failed to parse u32 out of input.");

            Instruction {
                direction: dir,
                clicks,
            }
        })
        .collect::<Vec<_>>();

    let mut dial = Dial::new();

    for instruction in instructions {
        dial.turn(instruction);
    }

    dial.password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        let input = include_str!("./sample.txt");
        let result = part_1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn part_1_test() {
        let input = include_str!("./input.txt");
        let result = part_1(input);
        println!("Part 1 result: {}", result);
    }
}
