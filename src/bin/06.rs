advent_of_code::solution!(6);

#[derive(Debug)]
struct Homework {
    data: Vec<u64>,
    ops: Vec<char>,
}

impl Homework {
    fn parse(input: &str) -> Self {
        let mut data = Vec::new();
        let mut ops = Vec::new();

        for line in input.lines() {
            for number_or_op in line.split_whitespace() {
                match number_or_op.parse() {
                    Ok(n) => data.push(n),
                    Err(_) => ops.push(number_or_op.chars().next().unwrap()),
                }
            }
        }
        Homework { data, ops }
    }

    fn part1(&self) -> u64 {
        let mut sum = 0;
        for (i, op) in self.ops.iter().enumerate() {
            let column_data = self.data.iter().skip(i).step_by(self.ops.len());
            let column_result: u64 = match op {
                '+' => column_data.sum(),
                '*' => column_data.product(),
                unknown_op => panic!("unknown op {}", unknown_op),
            };
            sum += column_result;
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let homework = Homework::parse(input);
    Some(homework.part1())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut sum = 0;

    // do some right padding to fix 'missing spaces'
    let longest_row = map.iter().map(|row| row.len()).max().unwrap();
    for row in map.iter_mut() {
        while row.len() < longest_row {
            row.push(b' ');
        }
    }
    let row_count = map.len();
    let mut column: i64 = (map[0].len() - 1).try_into().unwrap();
    let mut numbers = Vec::new();
    while column >= 0 {
        let mut number = Vec::new();
        // go over rows to build number but do not care about ops?
        for row in 0..(row_count - 1) {
            match map[row][column as usize] {
                b' ' => {}
                b'0'..=b'9' => {
                    let val = map[row][column as usize] - b'0';
                    number.push(val);
                }
                unexpected_value => {
                    panic!("Unexpected value {}", unexpected_value)
                }
            }
        }
        numbers.push(number);
        let op = map[map.len() - 1][column as usize];
        if op != b' ' {
            // op column has a op value, calc numbers
            let numbers_converted = numbers
                .iter()
                .map(|number: &Vec<u8>| number.iter().fold(0, |acc, n| acc * 10 + *n as u64));

            let result: u64 = match op {
                b'+' => numbers_converted.sum(),
                b'*' => numbers_converted.product(),
                _ => panic!("Unexpected op {}", op),
            };

            // sum + reset
            sum += result;
            numbers.clear();
            // next column is empty, so skip it
            column -= 1;
        }
        column -= 1;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
