advent_of_code::solution!(8);

type JunctionBoxLocation = (u64, u64, u64);

fn parse(input: &str) -> Vec<JunctionBoxLocation> {
    input
        .lines()
        .map(|line| line.split(',').map(|coord| coord.parse::<u64>().unwrap()))
        .map(|mut coords| {
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}

fn euclidean_distance_squared(a: &JunctionBoxLocation, b: &JunctionBoxLocation) -> u128 {
    let dx = a.0.abs_diff(b.0) as u128;
    let dy = a.1.abs_diff(b.1) as u128;
    let dz = a.2.abs_diff(b.2) as u128;
    dx * dx + dy * dy + dz * dz
}

fn calc_distances(junction_box_locations: &[JunctionBoxLocation]) -> Vec<(u128, usize, usize)> {
    let num_locations = junction_box_locations.len();
    let mut distances = Vec::new();
    for i in 0..num_locations {
        for j in (i + 1)..num_locations {
            let dist =
                euclidean_distance_squared(&junction_box_locations[i], &junction_box_locations[j]);
            distances.push((dist, i, j));
        }
    }

    distances.sort_by_key(|k| k.0);
    distances
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_box_locations = parse(input);
    let num_locations = junction_box_locations.len();
    let distances = calc_distances(&junction_box_locations);

    let num_connections = if num_locations == 20 { 10 } else { 1000 };

    let mut dsu = Dsu::new(num_locations);
    for &(_dist, i, j) in distances.iter().take(num_connections) {
        dsu.union(i, j);
    }

    let mut circuit_sizes = std::collections::HashMap::new();
    for i in 0..num_locations {
        let root = dsu.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<u64> = circuit_sizes.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    Some(sizes.iter().take(3).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_box_locations = parse(input);
    let num_locations = junction_box_locations.len();
    let distances = calc_distances(&junction_box_locations);

    let mut dsu = Dsu::new(num_locations);
    let mut num_circuits = num_locations;

    for &(_dist, i, j) in &distances {
        if dsu.find(i) != dsu.find(j) {
            dsu.union(i, j);
            num_circuits -= 1;
            if num_circuits == 1 {
                let x1 = junction_box_locations[i].0;
                let x2 = junction_box_locations[j].0;
                return Some(x1 * x2);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
