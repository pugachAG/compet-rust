use super::str::Str;

pub struct GridCoordinates {
    n: usize,
    m: usize,
}

pub trait Grid<T> {
    fn at(&self, i: usize, j: usize) -> &T;
    fn at_mut(&mut self, i: usize, j: usize) -> &mut T;
}

impl GridCoordinates {
    pub fn new(n: usize, m: usize) -> Self {
        Self { n, m }
    }

    pub fn len(&self) -> usize {
        self.n * self.m
    }

    #[inline]
    pub fn left_neighbour(&self, v: usize) -> Option<usize> {
        if v % self.m > 0 {
            Some(v - 1)
        } else {
            None
        }
    }

    #[inline]
    pub fn right_neightbour(&self, v: usize) -> Option<usize> {
        if v % self.m + 1 < self.m {
            Some(v + 1)
        } else {
            None
        }
    }

    #[inline]
    pub fn top_neighbour(&self, v: usize) -> Option<usize> {
        v.checked_sub(self.m)
    }

    #[inline]
    pub fn bottom_neighbour(&self, v: usize) -> Option<usize> {
        if v + self.m < self.len() {
            Some(v + self.m)
        } else {
            None
        }
    }

    pub fn side_neightbours(&self, v: usize) -> impl Iterator<Item = usize> {
        [
            self.left_neighbour(v),
            self.top_neighbour(v),
            self.right_neightbour(v),
            self.bottom_neighbour(v),
        ]
        .into_iter()
        .flat_map(|o| o)
    }

    pub fn to_point(&self, v: usize) -> (usize, usize) {
        (v / self.m, v % self.m)
    }

    pub fn from_point(&self, i: usize, j: usize) -> usize {
        i * self.m + j
    }

    pub fn at<'a, T>(&self, grid: &'a impl Grid<T>, v: usize) -> &'a T {
        let (i, j) = self.to_point(v);
        grid.at(i, j)
    }

    pub fn at_mut<'a, T>(&self, grid: &'a mut impl Grid<T>, v: usize) -> &'a mut T {
        let (i, j) = self.to_point(v);
        grid.at_mut(i, j)
    }
}

impl<T> Grid<T> for Vec<Vec<T>> {
    fn at(&self, i: usize, j: usize) -> &T {
        &self[i][j]
    }

    fn at_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self[i][j]
    }
}

impl Grid<u8> for Vec<Str> {
    fn at(&self, i: usize, j: usize) -> &u8 {
        &self[i][j]
    }

    fn at_mut(&mut self, i: usize, j: usize) -> &mut u8 {
        &mut self[i][j]
    }
}
