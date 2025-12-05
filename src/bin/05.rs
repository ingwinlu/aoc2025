use std::collections::HashSet;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Inventory {
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}
impl Inventory {
    fn parse(input: &str) -> Self {
        let mut ranges = Vec::new();
        let mut ingredients = Vec::new();
        let mut parse_ranges = true;
        for line in input.lines() {
            if line.is_empty() {
                parse_ranges = false;
                continue;
            }
            if parse_ranges {
                let (start, end) = line.split_once('-').unwrap();
                ranges.push((start.parse().unwrap(), end.parse().unwrap()));
            } else {
                ingredients.push(line.parse().unwrap());
            }
        }
        Inventory {
            ranges,
            ingredients,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let inventory = Inventory::parse(input);
    let mut fresh_count = 0u64;
    for ingredient in inventory.ingredients {
        for (start, end) in inventory.ranges.iter() {
            if ingredient >= *start && ingredient <= *end {
                fresh_count += 1;
                break;
            }
        }
    }
    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let inventory = Inventory::parse(input);
    let mut ranges = inventory.ranges;

    // Sort by the start of the range.
    ranges.sort_unstable_by_key(|k| k.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut range_iter = ranges.into_iter();

    let first_range = range_iter.next().expect("ranges must not be empty");
    let mut current_start = first_range.0;
    let mut current_end = first_range.1;

    for (next_start, next_end) in range_iter {
        if next_start <= current_end {
            // The ranges overlap, merge them.
            current_end = current_end.max(next_end);
        } else {
            // The ranges do not overlap, push the merged range and start a new one.
            merged_ranges.push((current_start, current_end));
            current_start = next_start;
            current_end = next_end;
        }
    }
    merged_ranges.push((current_start, current_end));

    let total_length: u64 = merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    Some(total_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
