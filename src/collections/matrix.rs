use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<f64>>,
}

#[derive(Debug, PartialEq)]
pub enum MatrixConstructionError {
    RaggedVec,
    NoRows,
    NoCols,
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

impl TryFrom<Vec<Vec<f64>>> for Matrix {
    type Error = MatrixConstructionError;

    fn try_from(vec2d: Vec<Vec<f64>>) -> Result<Matrix, MatrixConstructionError> {
        let rows = vec2d.len();
        if rows == 0 {
            return Err(MatrixConstructionError::NoRows);
        }

        let mut matrix = Vec::with_capacity(rows);

        let cols = vec2d[0].len();
        if cols == 0 {
            return Err(MatrixConstructionError::NoCols);
        }

        for row in vec2d {
            if row.len() != cols {
                return Err(MatrixConstructionError::RaggedVec);
            }
            matrix.push(row.clone());
        }

        Ok(Matrix { rows, cols, matrix })
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
    fn create_new_matrix() {
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

    #[test]
    fn create_matrix_from_2d_vec() {
        let array = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let resulting_matrix = Matrix {
            rows: 3,
            cols: 2,
            matrix: vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]],
        };
        assert_eq!(Matrix::try_from(array).unwrap(), resulting_matrix);
    }

    #[test]
    fn create_matrix_from_ragged_2d_vec() {
        let array = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0], vec![6.0]];
        let result = Matrix::try_from(array);
        assert_eq!(result, Err(MatrixConstructionError::RaggedVec));
    }
}
