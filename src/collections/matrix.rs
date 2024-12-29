use std::convert::TryFrom;
use std::ops::{Index, IndexMut, Mul};

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

impl TryFrom<&Vec<Vec<f64>>> for Matrix {
    type Error = MatrixConstructionError;

    fn try_from(vec2d: &Vec<Vec<f64>>) -> Result<Matrix, MatrixConstructionError> {
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

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Self::Output {
        assert_eq!(self.cols, other.rows);
        let mut resulting_matrix = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    resulting_matrix[[i, j]] += self[[i, k]] * other[[k, j]];
                }
            }
        }
        resulting_matrix
    }
}

pub trait Tuple4 {
    fn to_tuple4(self) -> [f64; 4];
}

impl<T: Tuple4> From<T> for Matrix {
    type Output = Matrix;

    fn from(self: T) -> Self::Output {
        Matrix::try_from(self.to_tuple4().to_vec())
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
            matrix: array.clone(),
        };
        assert_eq!(Matrix::try_from(&array).unwrap(), resulting_matrix);
    }

    #[test]
    fn create_matrix_from_ragged_2d_vec() {
        let array = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0], vec![6.0]];
        let result = Matrix::try_from(&array);
        assert_eq!(result, Err(MatrixConstructionError::RaggedVec));
    }

    #[test]
    fn mul_two_matrices() {
        let matrix1 = Matrix::try_from(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();
        let matrix2 = Matrix::try_from(&vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ])
        .unwrap();
        let resulting_matrix = Matrix::try_from(&vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ])
        .unwrap();
        assert_eq!(matrix1 * &matrix2, resulting_matrix);
    }
}
