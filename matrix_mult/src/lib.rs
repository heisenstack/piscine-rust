use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0.get(n).unwrap().clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

impl <T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Clone + std::ops::Add<Output = T>, {
    type Output = Option<Matrix<T>>;
    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![self.0[0][0].clone(); other.number_of_cols()]; self.number_of_rows()];

        for i in 0..self.number_of_rows() {
            for j in 0..other.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    result[i][j] = result[i][j].clone() + (self.0[i][k].clone() * other.0[k][j].clone());
                }
            }
        }
        Some(Matrix(result))
    }
}