#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point<const N: usize, T: Copy> {
    pub coords: [T; N],
    pub names: [char; N],
}
impl<const N: usize, T: Copy> Point<N, T> {
    pub fn new(coords: [T; N]) -> Self {
        let mut iter = ['x', 'y', 'z', 'm', 'n'].into_iter().take(N);
        let names: [_; N] = std::array::from_fn(|_| iter.next().unwrap());

        Point { coords, names }
    }

    pub fn new_with_names(coords: [T; N], names: [char; N]) -> Self {
        Point { coords, names }
    }

    pub fn get(&self, coord: char) -> Option<T> {
        let index = self.names.iter().position(|&x| x == coord)?;
        Some(self.coords[index])
    }
}
