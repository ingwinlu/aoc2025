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
    pub fn from_input_with_transform<F>(input: &str, transform: F) -> Self
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
        assert_eq!(map2d.data().len(), 15 * 16);

        assert_eq!(map2d.get((0, 0)), Some(&b'.'));
        assert_eq!(map2d.get((7, 0)), Some(&b'S'));
        assert_eq!(map2d.get((7, 1)), Some(&b'|'));
        assert_eq!(map2d.get((55, 1)), None);
    }
}
