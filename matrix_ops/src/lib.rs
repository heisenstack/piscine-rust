use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);


impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;
    fn add(self, other: Self) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let result = self
            .0
            .iter()
            .zip(&other.0)
            .map(|(row1, row2)| row1.iter().zip(row2).map(|(a, b)| a.clone() + b.clone()).collect())
            .collect();
        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Self) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let result = self
            .0
            .iter()
            .zip(&other.0)
            .map(|(row1, row2)| row1.iter().zip(row2).map(|(a, b)| a.clone() - b.clone()).collect())
            .collect();
        Some(Matrix(result))
    }
}