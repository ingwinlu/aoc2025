advent_of_code::solution!(9);

type Point = (u64, u64);
type Edge = (Point, Point);

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| line.split(',').map(|part| part.parse::<u64>().unwrap()))
        .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
        .collect()
}

fn area_between_two_points(p1: &Point, p2: &Point) -> u64 {
    (1 + p1.0.abs_diff(p2.0)) * (1 + p1.1.abs_diff(p2.1))
}

pub fn part_one(input: &str) -> Option<u64> {
    let tile_locations = parse(input);
    if tile_locations.len() < 2 {
        return None;
    }

    let mut max_area = 0;
    for (i, p1) in tile_locations.iter().enumerate() {
        for p2 in tile_locations.iter().skip(i + 1) {
            let area = area_between_two_points(p1, p2);
            if area > max_area {
                max_area = area;
            }
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    fn get_edges(vertices: &[Point]) -> Vec<Edge> {
        (0..vertices.len())
            .map(|i| (vertices[i], vertices[(i + 1) % vertices.len()]))
            .collect()
    }

    fn is_on_segment(p: Point, a: Point, b: Point) -> bool {
        p.0 >= a.0.min(b.0) && p.0 <= a.0.max(b.0) && p.1 >= a.1.min(b.1) && p.1 <= a.1.max(b.1)
    }

    fn is_on_boundary(p: Point, edges: &[Edge]) -> bool {
        edges.iter().any(|edge| {
            let (p1, p2) = *edge;
            if p1.0 == p2.0 {
                // Vertical edge
                p.0 == p1.0 && is_on_segment(p, p1, p2)
            } else {
                // Horizontal edge
                p.1 == p1.1 && is_on_segment(p, p1, p2)
            }
        })
    }

    fn is_inside(p: Point, edges: &[Edge]) -> bool {
        if is_on_boundary(p, edges) {
            return true;
        }
        let mut intersections = 0;
        for edge in edges.iter() {
            let (p1, p2) = edge;
            if p1.1 == p2.1 {
                continue;
            }
            if (p.1 < p1.1) != (p.1 < p2.1) {
                let x_intersect = (p.1 as i128 - p1.1 as i128) * (p2.0 as i128 - p1.0 as i128)
                    / (p2.1 as i128 - p1.1 as i128)
                    + p1.0 as i128;
                if (p.0 as i128) < x_intersect {
                    intersections += 1;
                }
            }
        }
        intersections % 2 == 1
    }

    fn is_rect_inside(r_min: Point, r_max: Point, edges: &[Edge]) -> bool {
        let corners = [r_min, r_max, (r_min.0, r_max.1), (r_max.0, r_min.1)];
        if corners.iter().any(|c| !is_inside(*c, edges)) {
            return false;
        }

        for edge in edges.iter() {
            let (p1, p2) = *edge;
            // Check if polygon edge's interior intersects with rectangle's interior
            let r_min_x_int = r_min.0 + 1;
            let r_max_x_int = r_max.0 - 1;
            let r_min_y_int = r_min.1 + 1;
            let r_max_y_int = r_max.1 - 1;

            if p1.0 == p2.0 {
                // Vertical edge
                if p1.0 > r_min_x_int
                    && p1.0 < r_max_x_int
                    && p1.1.max(p2.1) > r_min_y_int
                    && p1.1.min(p2.1) < r_max_y_int
                {
                    return false;
                }
            } else {
                // Horizontal edge
                if p1.1 > r_min_y_int
                    && p1.1 < r_max_y_int
                    && p1.0.max(p2.0) > r_min_x_int
                    && p1.0.min(p2.0) < r_max_x_int
                {
                    return false;
                }
            }
        }
        true
    }

    let vertices = parse(input);
    if vertices.len() < 2 {
        return None;
    }
    let edges = get_edges(&vertices);
    let mut max_area = 0;

    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[j];

            let r_min = (p1.0.min(p2.0), p1.1.min(p2.1));
            let r_max = (p1.0.max(p2.0), p1.1.max(p2.1));

            if is_rect_inside(r_min, r_max, &edges) {
                let area = (1 + p1.0.abs_diff(p2.0)) * (1 + p1.1.abs_diff(p2.1));
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    Some(max_area)
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
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_area_between_two_corners() {
        assert_eq!(24, area_between_two_points(&(2, 5), &(9, 7)));
        assert_eq!(35, area_between_two_points(&(7, 1), &(11, 7)));
        assert_eq!(6, area_between_two_points(&(7, 3), &(2, 3)));
        assert_eq!(50, area_between_two_points(&(2, 5), &(11, 1)));
    }
}
