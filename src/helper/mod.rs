type Coords = (usize, usize);

pub struct Map2D<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Map2D<T> {
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn get(&self, (x, y): Coords) -> Option<&T> {
        if x >= self.width {
            return None;
        }
        self.data.get(x + self.width * y)
    }
}

// impl<'a, T: std::convert::From<&'a str>> Map2D<T> {
//     pub fn from_input(input: &'a str) -> Self {
//         let lines = input.lines();
//         let width = lines
//             .clone()
//             .next()
//             .map(|line| line.len())
//             .expect("Could not determine width");
//         let data: Vec<T> = lines.map(|line| line.into()).flatten().collect();
//         Self { data, width }
//     }
// }

impl<'a> Map2D<u8> {
    pub fn from_input_as_u8(input: &'a str) -> Self {
        let lines = input.lines();
        let width = lines
            .clone()
            .next()
            .map(|line| line.len())
            .expect("Could not determine width");
        let data: Vec<u8> = lines.flat_map(|line| line.bytes()).collect();
        let height = data.len() / width;
        Self {
            data,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map2d_from_input_as_u8() {
        let input = "\
.......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        let map2d: Map2D<u8> = Map2D::from_input_as_u8(input);
        assert_eq!(map2d.width, 15);
        assert_eq!(map2d.data.len(), 15 * 16);

        assert_eq!(map2d.get((0, 0)), Some(&b'.'));
        assert_eq!(map2d.get((7, 0)), Some(&b'S'));
        assert_eq!(map2d.get((7, 1)), Some(&b'|'));
        assert_eq!(map2d.get((55, 1)), None);
    }
}
