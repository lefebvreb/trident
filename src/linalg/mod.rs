use std::ops::Deref;

mod complex;
pub use complex::*;

#[derive(Clone, Debug)]
pub struct Matrix<const N: usize>  {
    data: [[c32; N]; N],
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self { data: [[c32::ZERO; N]; N] }
    }
}

impl<const N: usize> Matrix<N> {
    pub fn is_unitary(&self) -> bool {
        todo!()
    }

    pub fn as_unitary(self) -> Option<UnitaryMatrix<N>> {
        self.is_unitary().then(|| UnitaryMatrix::new_unchecked(self))
    }
}

#[derive(Clone, Debug)]
pub struct UnitaryMatrix<const N: usize> {
    mat: Matrix<N>
}

impl<const N: usize> Deref for UnitaryMatrix<N> {
    type Target = Matrix<N>;

    fn deref(&self) -> &Self::Target {
        &self.mat
    }
}

impl<const N: usize> UnitaryMatrix<N> {
    pub fn new_unchecked(mat: Matrix<N>) -> Self {
        Self { mat }
    }

    pub fn take(self) -> Matrix<N> {
        self.mat
    }
}