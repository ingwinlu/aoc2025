advent_of_code::solution!(1);

type Position = u64;
type Step = u64;

enum Direction {
    L,
    R,
}

fn parse_line(line: &str) -> (Direction, Step) {
    let steps = line[1..].parse::<Step>().unwrap();
    let dir = match line.as_bytes()[0] {
        b'L' => Direction::L,
        b'R' => Direction::R,
        _ => unreachable!(),
    };
    (dir, steps)
}

#[derive(Debug)]
struct SafeDial {
    current_position: Position,
    crossed_zero_counter: Position,
}

impl SafeDial {
    pub fn new() -> Self {
        Self {
            current_position: 50,
            crossed_zero_counter: 0,
        }
    }

    pub fn right(&mut self, steps: Step) -> &mut Self {
        self.crossed_zero_counter += (self.current_position + steps) / 100;
        self.current_position = (self.current_position + steps) % 100;
        self
    }

    pub fn left(&mut self, steps: Step) -> &mut Self {
        self.crossed_zero_counter += (99 - (self.current_position + 99) % 100 + steps) / 100;
        self.current_position = (100 + self.current_position - steps % 100) % 100;
        self
    }

    pub fn current_position(&self) -> Step {
        self.current_position
    }

    pub fn crossed_zero_counter(&self) -> Step {
        self.crossed_zero_counter
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut zero_counter = 0;
    let mut safe = SafeDial::new();
    for line in input.lines() {
        let (dir, steps) = parse_line(line);
        match dir {
            Direction::L => safe.left(steps),
            Direction::R => safe.right(steps),
        };
        if safe.current_position() == 0 {
            zero_counter += 1;
        }
    }
    Some(zero_counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut safe = SafeDial::new();
    for line in input.lines() {
        let (dir, steps) = parse_line(line);
        match dir {
            Direction::L => safe.left(steps),
            Direction::R => safe.right(steps),
        };
    }
    Some(safe.crossed_zero_counter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_left_by_one() {
        let mut safe = SafeDial::new();
        safe.left(1);
        assert_eq!(safe.current_position, 49);
    }

    #[test]
    fn test_turn_left_by_50() {
        let mut safe = SafeDial::new();
        safe.left(50);
        assert_eq!(safe.current_position, 0);
    }

    #[test]
    fn test_turn_left_by_51() {
        let mut safe = SafeDial::new();
        safe.left(51);
        assert_eq!(safe.current_position, 99);
    }

    #[test]
    fn test_turn_right_by_one() {
        let mut safe = SafeDial::new();
        safe.right(1);
        assert_eq!(safe.current_position, 51);
    }

    #[test]
    fn test_turn_right_by_50() {
        let mut safe = SafeDial::new();
        safe.right(50);
        assert_eq!(safe.current_position, 0);
    }

    #[test]
    fn test_turn_right_by_51() {
        let mut safe = SafeDial::new();
        safe.right(51);
        assert_eq!(safe.current_position, 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
