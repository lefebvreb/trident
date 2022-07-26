use std::ops::{Deref, Index, IndexMut};

mod complex;
pub use complex::*;

#[derive(Clone, Debug)]
pub struct Matrix<const N: usize> {
    data: [[c64; N]; N],
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self { data: [[c64::ZERO; N]; N] }
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [c64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> Matrix<N> {
    pub fn new(data: [[c64; N]; N]) -> Self {
        Self { data }
    }

    pub fn is_unitary(&self) -> bool {
        (0..N).zip((0..N)).all(|(i, j)| {
            let target = if i == j { c64::ONE } else { c64::ZERO };
            (0..N).map(|k| self[i][k] * self[j][k]).sum::<c64>() == target
        })
    }

    pub fn as_unitary(self) -> Option<UnitaryMatrix<N>> {
        self.is_unitary().then(|| UnitaryMatrix::new_unchecked(self))
    }
}

impl Matrix<2> {
    pub fn new2(u00: c64, u01: c64, u10: c64, u11: c64) -> Self {
        Self { data: [[u00, u01], [u10, u11]] }
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