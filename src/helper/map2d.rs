// 1. Define the parsing behavior
pub trait ParseMapTile {
    fn from_char(c: char) -> Self;
}

// 2. Implement for your desired types
impl ParseMapTile for char {
    fn from_char(c: char) -> Self {
        c
    }
}

impl ParseMapTile for u8 {
    fn from_char(c: char) -> Self {
        c as u8
    }
}

impl ParseMapTile for u32 {
    fn from_char(c: char) -> Self {
        c.to_digit(10).expect("Input contained non-digit char")
    }
}

type Coords = (usize, usize);

pub struct Map2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Map2D<T> {
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

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
    // We accept a closure `F` that takes a char and returns a T
    fn from_input_with_transform<F>(input: &str, transform: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let width = input.lines().next().map_or(0, |line| line.len());

        let data: Vec<T> = input
            .lines()
            .flat_map(|line| line.chars().map(&transform))
            .collect();

        // Avoid division by zero if input is empty
        let height = if width > 0 { data.len() / width } else { 0 };

        Self {
            data,
            width,
            height,
        }
    }
}

impl<T: ParseMapTile> Map2D<T> {
    pub fn from_input(input: &str) -> Self {
        // We reuse the transform logic here directly
        Self::from_input_with_transform(input, T::from_char)
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
        let map2d: Map2D<u8> = Map2D::<u8>::from_input(input);
        assert_eq!(map2d.width(), 15);
        assert_eq!(map2d.height(), 16);
        assert_eq!(map2d.data().len(), 15 * 16);

        assert_eq!(map2d.get((0, 0)), Some(&b'.'));
        assert_eq!(map2d.get((7, 0)), Some(&b'S'));
        assert_eq!(map2d.get((7, 1)), Some(&b'|'));
        assert_eq!(map2d.get((15, 0)), None); // x out of bounds
        assert_eq!(map2d.get((0, 16)), None); // y out of bounds
    }

    #[test]
    fn test_map2d_from_input_with_transform() {
        let input = "123\n456";
        let map2d = Map2D::from_input_with_transform(input, |c| c.to_digit(10).unwrap());

        assert_eq!(map2d.width(), 3);
        assert_eq!(map2d.height(), 2);
        assert_eq!(map2d.data().len(), 6);
        assert_eq!(map2d.get((0, 0)), Some(&1));
        assert_eq!(map2d.get((1, 0)), Some(&2));
        assert_eq!(map2d.get((2, 0)), Some(&3));
        assert_eq!(map2d.get((0, 1)), Some(&4));
        assert_eq!(map2d.get((1, 1)), Some(&5));
        assert_eq!(map2d.get((2, 1)), Some(&6));
        assert_eq!(map2d.get((3, 0)), None);
        assert_eq!(map2d.get((0, 2)), None);
    }

    #[test]
    fn test_map2d_from_input_as_u32() {
        let input = "987\n654";
        let map2d = Map2D::<u32>::from_input(input);

        assert_eq!(map2d.width(), 3);
        assert_eq!(map2d.height(), 2);
        assert_eq!(*map2d.data(), vec![9, 8, 7, 6, 5, 4]);
        assert_eq!(map2d.get((1, 0)), Some(&8));
        assert_eq!(map2d.get((2, 1)), Some(&4));
    }

    #[test]
    #[should_panic(expected = "Input contained non-digit char")]
    fn test_map2d_from_input_as_u32_panic() {
        let input = "12a\n345";
        Map2D::<u32>::from_input(input);
    }

    #[test]
    fn test_map2d_empty_input() {
        let input = "";
        let map2d = Map2D::<char>::from_input(input);
        assert_eq!(map2d.width(), 0);
        assert_eq!(map2d.height(), 0);
        assert_eq!(map2d.data().len(), 0);
        assert_eq!(map2d.get((0, 0)), None);
    }

    #[test]
    fn test_map2d_input_with_trailing_newline() {
        let input = "1\n2\n";
        let map2d = Map2D::<u32>::from_input(input);
        assert_eq!(map2d.width(), 1);
        assert_eq!(map2d.height(), 2);
        assert_eq!(map2d.get((0, 0)), Some(&1));
        assert_eq!(map2d.get((0, 1)), Some(&2));
    }

    #[test]
    fn test_map2d_irregular_input() {
        let input = "12\n345";
        let map2d = Map2D::<u32>::from_input(input);
        assert_eq!(map2d.width(), 2); // from "12"
        assert_eq!(map2d.data().len(), 5); // 1,2,3,4,5
        assert_eq!(map2d.height(), 2); // 5 / 2 = 2
        assert_eq!(map2d.get((0, 0)), Some(&1));
        assert_eq!(map2d.get((1, 0)), Some(&2));
        assert_eq!(map2d.get((0, 1)), Some(&3));
        assert_eq!(map2d.get((1, 1)), Some(&4));
        // The '5' is at index 4.
        // get(0,2) -> 0 + 2 * 2 = 4.
        assert_eq!(map2d.get((0, 2)), Some(&5));
        // get(1,2) -> 1 + 2 * 2 = 5, out of bounds.
        assert_eq!(map2d.get((1, 2)), None);
    }

    #[test]
    fn test_map2d_from_input_as_char() {
        let input = "a b\nc d";
        let map2d = Map2D::<char>::from_input(input);

        assert_eq!(map2d.width(), 3);
        assert_eq!(map2d.height(), 2);
        assert_eq!(*map2d.get((0, 0)).unwrap(), 'a');
        assert_eq!(*map2d.get((1, 0)).unwrap(), ' ');
        assert_eq!(*map2d.get((2, 0)).unwrap(), 'b');
        assert_eq!(*map2d.get((0, 1)).unwrap(), 'c');
        assert_eq!(*map2d.get((1, 1)).unwrap(), ' ');
        assert_eq!(*map2d.get((2, 1)).unwrap(), 'd');
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Tiles {
        Empty,
        Visited,
        Splitter,
        Start,
    }

    impl ParseMapTile for Tiles {
        fn from_char(c: char) -> Self {
            match c {
                '.' => Tiles::Empty,
                '^' => Tiles::Splitter,
                '|' => Tiles::Visited,
                'S' => Tiles::Start,
                _ => panic!("Unknown tile: {}", c),
            }
        }
    }

    #[test]
    fn test_map2d_from_input_enum() {
        let input = "S^|\n.|.";
        let map2d: Map2D<Tiles> = Map2D::from_input(input);

        assert_eq!(map2d.width(), 3);
        assert_eq!(map2d.height(), 2);
        assert_eq!(*map2d.get((0, 0)).unwrap(), Tiles::Start);
        assert_eq!(*map2d.get((1, 0)).unwrap(), Tiles::Splitter);
        assert_eq!(*map2d.get((2, 0)).unwrap(), Tiles::Visited);
        assert_eq!(*map2d.get((0, 1)).unwrap(), Tiles::Empty);
        assert_eq!(*map2d.get((1, 1)).unwrap(), Tiles::Visited);
        assert_eq!(*map2d.get((2, 1)).unwrap(), Tiles::Empty);
    }
}
