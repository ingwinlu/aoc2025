advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Empty,
    Paper,
}

#[derive(Debug)]
struct Map {
    fields: Vec<Vec<Field>>,
}

type Coords = (usize, usize);

impl Map {
    fn from_str(input: &str) -> Self {
        let fields = input
            .lines()
            .map(|line| {
                line.trim()
                    .bytes()
                    .map(|b| match b {
                        b'.' => Field::Empty,
                        b'@' => Field::Paper,
                        unknown => panic!("Unknown character: {:?}", unknown as char),
                    })
                    .collect()
            })
            .collect();
        Self { fields }
    }

    fn get_neighbors(&self, (x, y): Coords) -> Vec<Coords> {
        let mut neighbors = Vec::new();
        if self.fields.is_empty() {
            return neighbors;
        }
        let (rows, cols) = (self.fields.len() as isize, self.fields[0].len() as isize);

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < rows && ny >= 0 && ny < cols {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }
        neighbors
    }

    fn empty_at_coords(&mut self, coords: Vec<(usize, usize)>) {
        for (x, y) in coords {
            self.fields[x][y] = Field::Empty;
        }
    }

    fn get_paper_neighbors(&self, (x, y): Coords) -> u8 {
        self.get_neighbors((x, y))
            .iter()
            .map(|(x, y)| match self.fields[*x][*y] {
                Field::Empty => 0u8,
                Field::Paper => 1u8,
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = Map::from_str(input);
    let mut movable_paper = 0;
    for (x, row) in map.fields.iter().enumerate() {
        for (y, field) in row.iter().enumerate() {
            if field == &Field::Empty {
                continue;
            }
            if map.get_paper_neighbors((x, y)) < 4 {
                movable_paper += 1;
            }
        }
    }
    Some(movable_paper)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::from_str(input);
    let mut movable_paper = 0;
    loop {
        let mut new_removed_paper_coords: Vec<Coords> = Vec::new();
        for (x, row) in map.fields.iter().enumerate() {
            for (y, field) in row.iter().enumerate() {
                if field == &Field::Empty {
                    continue;
                }
                if map.get_paper_neighbors((x, y)) < 4 {
                    new_removed_paper_coords.push((x, y));
                }
            }
        }

        if new_removed_paper_coords.is_empty() {
            break;
        }
        movable_paper += new_removed_paper_coords.len() as u64;
        map.empty_at_coords(new_removed_paper_coords);
    }
    Some(movable_paper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
