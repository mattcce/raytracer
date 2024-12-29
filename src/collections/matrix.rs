use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<f64>>,
}

type Idx = [usize; 2];

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

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl From<&Vec<Vec<f64>>> for Matrix {
    // does not consume the vector and requires cloning
    fn from(vec2d: &Vec<Vec<f64>>) -> Self {
        let rows = vec2d.len();
        assert_ne!(rows, 0);

        let mut matrix = Vec::with_capacity(rows);

        let cols = vec2d[0].len();
        assert_ne!(rows, 0);

        // as we need to check row lengths anyway, we can clone while we're are it
        for row in vec2d {
            assert_eq!(row.len(), cols);
            matrix.push(row.clone());
        }

        Matrix { rows, cols, matrix }
    }
}

impl Index<Idx> for Matrix {
    type Output = f64;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.matrix[index[0]][index[1]]
    }
}

impl IndexMut<Idx> for Matrix {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
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

pub trait Tuple4: Copy + From<Matrix> {
    fn to_tuple4(self) -> [f64; 4];
}

impl<T: Tuple4> From<T> for Matrix {
    fn from(value: T) -> Self {
        let tuple: Vec<Vec<f64>> = value.to_tuple4().into_iter().map(|x| vec![x]).collect();
        Matrix::from(&tuple)
    }
}

impl Matrix {
    pub fn transpose(&self) -> Matrix {
        let (tm_rows, tm_cols) = (self.cols, self.rows);
        let mut transposed_matrix = Matrix::new(tm_rows, tm_cols);
        for i in 0..tm_rows {
            for j in 0..tm_cols {
                transposed_matrix[[i, j]] = self[[j, i]];
            }
        }
        transposed_matrix
    }

    pub fn det(&self) -> f64 {
        assert_eq!(self.rows, self.cols);
        assert!(self.rows >= 2);

        if self.rows == 2 {
            self[[0, 0]] * self[[1, 1]] - self[[0, 1]] * self[[1, 0]]
        } else {
            let mut det = 0.0;
            for i in 0..self.rows {
                det += self[[0, i]] * self.cofactor([0, i]);
            }
            det
        }
    }

    pub fn submatrix(&self, [sm_row, sm_col]: Idx) -> Matrix {
        let mut submatrix = self.matrix.clone();

        submatrix.remove(sm_row);
        submatrix.iter_mut().for_each(|row| {
            row.remove(sm_col);
            row.shrink_to_fit();
        });

        Matrix {
            rows: self.rows - 1,
            cols: self.cols - 1,
            matrix: submatrix,
        }
    }

    pub fn minor(&self, index: Idx) -> f64 {
        self.submatrix(index).det()
    }

    pub fn cofactor(&self, index @ [row, col]: Idx) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(index)
        } else {
            -self.minor(index)
        }
    }

    pub fn invert(&self) -> Matrix {
        let (rows, cols) = (self.rows, self.cols);
        // panics if determinant is uncomputable (non-square matrix), checked by .det() method
        let det = self.det();
        assert_ne!(det, 0.0);

        let mut inverse_matrix = Matrix::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                // implicit transpose
                inverse_matrix[[j, i]] = self.cofactor([i, j]) / det;
            }
        }

        inverse_matrix
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
    fn matrix_accessors() {
        let matrix = Matrix::new(4, 1);
        assert_eq!(matrix.rows(), 4);
        assert_eq!(matrix.cols(), 1);
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
        assert_eq!(Matrix::from(&array), resulting_matrix);
    }

    #[test]
    #[should_panic]
    fn create_matrix_from_ragged_2d_vec() {
        let array = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0], vec![6.0]];
        let _result = Matrix::from(&array);
    }

    #[test]
    fn mul_two_matrices() {
        let matrix1 = Matrix::from(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix2 = Matrix::from(&vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let resulting_matrix = Matrix::from(&vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(matrix1 * &matrix2, resulting_matrix);
    }

    use super::super::{Point, Vector};

    #[test]
    fn point_to_matrix() {
        let point = Point::new(6.0, 4.0, 2.0);
        let matrix = Matrix {
            rows: 4,
            cols: 1,
            matrix: vec![vec![6.0], vec![4.0], vec![2.0], vec![1.0]],
        };
        assert_eq!(Matrix::from(point), matrix);
    }

    #[test]
    fn vector_to_matrix() {
        let vector = Vector::new(6.0, 4.0, 2.0);
        let matrix = Matrix {
            rows: 4,
            cols: 1,
            matrix: vec![vec![6.0], vec![4.0], vec![2.0], vec![0.0]],
        };
        assert_eq!(Matrix::from(vector), matrix);
    }

    #[test]
    fn transpose_matrix() {
        let matrix = Matrix::from(&vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let resulting_matrix = Matrix::from(&vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(matrix.transpose(), resulting_matrix);
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        let matrix = Matrix::from(&vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert_eq!(matrix.det(), 17.0);
    }

    #[test]
    fn submatrix_of_matrix() {
        let matrix = Matrix::from(&vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let resulting_submatrix = Matrix::from(&vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);
        assert_eq!(matrix.submatrix([2, 1]), resulting_submatrix);
    }

    #[test]
    fn minor_of_matrix() {
        let matrix = Matrix::from(&vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(matrix.minor([1, 0]), 25.0);
    }

    #[test]
    fn cofactor_of_matrix() {
        let matrix = Matrix::from(&vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(matrix.cofactor([0, 0]), -12.0);
        assert_eq!(matrix.cofactor([1, 0]), -25.0);
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let matrix = Matrix::from(&vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(matrix.cofactor([0, 0]), 690.0);
        assert_eq!(matrix.cofactor([0, 1]), 447.0);
        assert_eq!(matrix.cofactor([0, 2]), 210.0);
        assert_eq!(matrix.cofactor([0, 3]), 51.0);
        assert_eq!(matrix.det(), -4071.0);
    }

    #[test]
    fn inverse_of_identity_matrix() {
        let matrix = Matrix::from(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(matrix.invert(), matrix);
    }
}
