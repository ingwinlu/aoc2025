advent_of_code::solution!(9);

type Coords = (u64, u64);

fn parse(input: &str) -> Vec<Coords> {
    input
        .lines()
        .map(|line| line.split(',').map(|part| part.parse::<u64>().unwrap()))
        .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
        .collect()
}

fn area_between_two_corners(corner1: &Coords, corner2: &Coords) -> u64 {
    let x = 1 + corner1.0.abs_diff(corner2.0);
    let y = 1 + corner1.1.abs_diff(corner2.1);
    x * y
}

pub fn part_one(input: &str) -> Option<u64> {
    let tile_locations = parse(input);
    let mut max_area = 0;

    if tile_locations.len() < 2 {
        return None;
    }

    for (i, corner1) in tile_locations.iter().enumerate() {
        for corner2 in tile_locations.iter().skip(i + 1) {
            let area = area_between_two_corners(corner1, corner2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_area_between_two_corners() {
        assert_eq!(24, area_between_two_corners(&(2, 5), &(9, 7)));
        assert_eq!(35, area_between_two_corners(&(7, 1), &(11, 7)));
        assert_eq!(6, area_between_two_corners(&(7, 3), &(2, 3)));
        assert_eq!(50, area_between_two_corners(&(2, 5), &(11, 1)));
    }
}
