use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let mut matrix = Vec::with_capacity(rows);
        for _i_row in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _i_col in 0..cols {
                row.push(0.0);
            }
            matrix.push(row);
        }
        Matrix { rows, cols, matrix }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_matrix() {
        let matrix = Matrix::new(3, 5);
        let stored_matrix = vec![
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0],
        ];
        let resulting_matrix = Matrix {
            rows: 3,
            cols: 5,
            matrix: stored_matrix,
        };
        assert_eq!(matrix, resulting_matrix);
    }

    #[test]
    fn index_and_modify_matrix() {
        let mut matrix = Matrix::new(3, 5);
        assert_eq!(matrix[[2, 1]], 0.0);
        matrix[[2, 3]] = 64.0;
        assert_eq!(matrix[[2, 3]], 64.0);
    }
}
