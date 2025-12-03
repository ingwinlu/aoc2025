advent_of_code::solution!(3);

fn parse_numbers(line: &str) -> Vec<u8> {
    line.bytes().map(|n| n - 48).collect()
}

fn calc_battery_power_1(a: u8, b: u8) -> u8 {
    a * 10 + b
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let numbers = parse_numbers(line);
        let mut best = 0;
        for i in 0..numbers.len() - 1 {
            if (numbers[i] * 10) < best {
                // can not exceed best, skip
                continue;
            }
            for j in (i + 1)..numbers.len() {
                let new_power = calc_battery_power_1(numbers[i], numbers[j]);
                if new_power > best {
                    best = new_power;
                }
            }
        }
        sum += best as u64;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers = parse_numbers(line);
                let n = numbers.len();
                if n < 12 {
                    return 0;
                }

                let mut best = 0u64;
                let mut start_index = 0;
                for i in 0..12 {
                    let digits_left_to_pick = 12 - i;
                    let end_search_index = n - digits_left_to_pick;

                    let mut best_val_for_pos = 0;
                    let mut best_val_index = 0;

                    for (j, &val) in numbers
                        .iter()
                        .enumerate()
                        .take(end_search_index + 1)
                        .skip(start_index)
                    {
                        if val > best_val_for_pos {
                            best_val_for_pos = val;
                            best_val_index = j;
                            if best_val_for_pos == 9 {
                                break;
                            }
                        }
                    }

                    best = best * 10 + best_val_for_pos as u64;
                    start_index = best_val_index + 1;
                }
                best
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
