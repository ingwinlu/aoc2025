advent_of_code::solution!(2);

fn split_range_of_ids(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(a, b)| {
            (
                a.parse()
                    .unwrap_or_else(|e| panic!("Failed to parse '{a}': {e}")),
                b.parse()
                    .unwrap_or_else(|e| panic!("Failed to parse '{b}': {e}")),
            )
        })
        .collect()
}

fn contains_invalid_sequence_1(sequence: u64) -> bool {
    let s = sequence.to_string().bytes().collect::<Vec<u8>>();
    let s_length = s.len();
    if s_length % 2 == 1 {
        return false;
    }
    for (h1, h2) in s[0..s_length / 2].iter().zip(s[s_length / 2..].iter()) {
        if h1 != h2 {
            return false;
        }
    }
    true
}

fn solver(input: &str, is_invalid: fn(u64) -> bool) -> Option<u64> {
    let mut sum = 0;
    for (range_start, range_end) in split_range_of_ids(input) {
        for i in range_start..=range_end {
            if is_invalid(i) {
                sum += i;
            }
        }
    }
    Some(sum)
}

pub fn part_one(input: &str) -> Option<u64> {
    solver(input, contains_invalid_sequence_1)
}

// The original implementation, renamed.
#[allow(dead_code)]
fn contains_invalid_sequence_2_string(input: u64) -> bool {
    let s = input.to_string();
    let n = s.len();

    // The length of the repeating part can be from 1 to n/2.
    for len in 1..=n / 2 {
        // The total length must be a multiple of the sub-sequence length.
        if n.is_multiple_of(len) {
            let repetitions = n / len;
            // The problem says "repeated at least twice".
            if repetitions >= 2 {
                let pattern = &s[0..len];
                if pattern.repeat(repetitions) == s {
                    return true; // Found a repeating pattern, so it's invalid.
                }
            }
        }
    }

    false // No repeating pattern found, so it's valid.
}

fn contains_invalid_sequence_2(input: u64) -> bool {
    if input < 10 {
        return false;
    }

    let mut num_digits = 0u32;
    {
        // Calculate the number of digits in the input.
        let mut temp = input;
        while temp > 0 {
            temp /= 10;
            num_digits += 1;
        }
    }

    // No pattern can be longer than half the input
    for len in 1..=num_digits / 2 {
        // pattern needs to 'fit' exactly into input
        if num_digits.is_multiple_of(len) {
            // construct pattern by "cutting off" at pattern length
            let ten_pow_len = 10u64.pow(len);
            let pattern_val = input / 10u64.pow(num_digits - len);

            // build a 'solution' where we repeat the pattern
            let mut reconstructed = 0u64;
            let repetitions = num_digits / len;
            for _ in 0..repetitions {
                reconstructed = reconstructed * ten_pow_len + pattern_val
            }
            if reconstructed == input {
                return true;
            }
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    solver(input, contains_invalid_sequence_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_implementations_consistency() {
        for i in 1..1_000_000 {
            let string_result = contains_invalid_sequence_2_string(i);
            let math_result = contains_invalid_sequence_2(i);
            assert_eq!(string_result, math_result, "Mismatch for number {}", i);
        }
    }
}
