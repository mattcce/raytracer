use std::ops::{Index, IndexMut};

pub struct Matrix {
    rows: usize,
    columns: usize,
    matrix: Vec<Vec<f64>>,
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.matrix[index[0]][index[1]]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.matrix[index[0]][index[1]]
    }
}
