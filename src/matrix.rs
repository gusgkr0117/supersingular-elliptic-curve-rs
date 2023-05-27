use std::ops::Add;
use num::Num;

#[derive(Clone)]
pub struct Matrix<T> where T : Num {
    pub element : Vec<T>,
    nrow : usize,
    ncol : usize,
}

impl<T> Matrix<T> where T: Num + Clone {
    pub fn new(nrow : usize, ncol : usize) -> Self {
        Matrix{
            element : vec![T::zero().clone() ; nrow * ncol],
            nrow : nrow,
            ncol : ncol,
        }
    }
}

impl<T> Add<Matrix<T>> for Matrix<T> where T: Num + Clone {
    type Output = Matrix<T>;
    fn add(self, rhs : Matrix<T>) -> Matrix<T> {
        assert!((self.nrow, self.ncol) == (rhs.nrow, rhs.ncol));
        let mut result : Matrix<T> = Matrix::new(self.nrow, self.ncol);
        for i in 0..result.nrow {
            for j in 0..result.ncol {
                result.element[i * result.ncol + j] = self.element[i * result.ncol + j].clone() + rhs.element[i * result.ncol + j].clone();
            }
        }
        result
    }
}