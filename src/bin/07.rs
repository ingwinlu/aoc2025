use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::helper::Map2D;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map2D::from_input_as_u8(input);
    // find start
    let starting_position = map
        .data
        .iter()
        .position(|&x| x == b'S')
        .map(|x| (x % map.width, x / map.width))
        .unwrap();

    let mut split_count = 0;

    let mut beams = vec![starting_position];
    let mut visited = HashSet::new();
    visited.insert(starting_position);

    while let Some(beam) = beams.pop() {
        // step down
        let new_beam_position = (beam.0, beam.1 + 1);
        if new_beam_position.1 >= map.height {
            // end of map
            continue;
        }

        match map.get(new_beam_position) {
            Some(b'.') => {
                if visited.insert(new_beam_position) {
                    beams.push(new_beam_position);
                }
            }
            Some(b'^') => {
                split_count += 1;
                if beam.0 > 0 {
                    let left = (beam.0 - 1, beam.1);
                    if visited.insert(left) {
                        beams.push(left);
                    }
                }
                if beam.0 < map.width - 1 {
                    let right = (beam.0 + 1, beam.1);
                    if visited.insert(right) {
                        beams.push(right);
                    }
                }
            }
            unexpected => panic!("Unexpected value: {:?}", unexpected),
        }
        // println!("beams: {:?}, {:?}", beams, split_count);
    }
    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map2D::from_input_as_u8(input);
    let starting_position = map
        .data
        .iter()
        .position(|&x| x == b'S')
        .map(|x| (x % map.width, x / map.width))
        .unwrap();

    let mut cache = HashMap::new();
    let split_count = count_splits(starting_position, &map, &mut cache);

    // Add +1 as we dont need split count but also the original 'timeline'
    Some(split_count + 1)
}

fn count_splits(
    pos: (usize, usize),
    map: &Map2D<u8>,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&pos) {
        return cached;
    }

    let (x, y) = pos;
    if y >= map.height {
        return 0;
    }

    let result = match map.get((x, y)) {
        Some(b'.') => count_splits((x, y + 1), map, cache),
        Some(b'^') => {
            let mut timelines = 1;
            if x > 0 {
                timelines += count_splits((x - 1, y), map, cache);
            }
            if x < map.width - 1 {
                timelines += count_splits((x + 1, y), map, cache);
            }
            timelines
        }
        Some(b'S') => count_splits((x, y + 1), map, cache),
        _ => 0,
    };

    cache.insert(pos, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
