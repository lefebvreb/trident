use std::ops::{Deref, Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, Neg};

use thiserror::Error;

use crate::linalg::c64;

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
    pub const fn new(data: [[c64; N]; N]) -> Self {
        Self { data }
    }

    pub const fn eye() -> Self {
        // TODO: make this less ugly when more const fn constructs get stabilized
        let mut data = [[c64::ZERO; N]; N];
        let mut i = 0;
        while i < N {
            data[i][i] = c64::ONE;
            i += 1;
        }
        Self::new(data)
    }

    pub const fn raw(&self) -> &[[c64; N]; N] {
        &self.data
    }

    pub fn raw_mut(&mut self) -> &mut [[c64; N]; N] {
        &mut self.data
    }

    pub fn is_unitary(&self) -> bool {
        (0..N).all(|i| (i..N).all(|j| {
            let target = if i == j { c64::ONE } else { c64::ZERO };
            (0..N).map(|k| self[i][k] * self[j][k]).sum::<c64>() == target
        }))
    }

    pub fn as_unitary(self) -> Option<UnitaryMatrix<N>> {
        self.try_into().ok()
    }

    // TODO: replace the previous implementations of krenecker by this one, when
    // the following construct becomes stable
    // pub fn kronecker<const M: usize>(&self, rhs: &Matrix<M>) -> Matrix<{N * M}> {
    //     ...
    // }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, rhs: &Self) -> bool {
        (0..N).all(|i| (0..N).all(|j| {
            self[i][j] == rhs[i][j]
        }))
    }
}

impl Matrix<2> {
    pub const fn new2x2(u00: c64, u01: c64, u10: c64, u11: c64) -> Self {
        Self::new([[u00, u01], [u10, u11]])
    }

    pub fn inv2x2(&self) -> Self {
        let [[u00, u01], [u10, u11]] = self.data;
        let det = (u00 * u11 - u01 * u10).recip();
        Matrix::new2x2(u11 * det, -u01 * det, -u10 * det, u00 * det)
    }

    pub fn kronecker(&self, rhs: &Self) -> Matrix<4> {
        let mut res = Matrix::default();

        (0..2).for_each(|i| (0..2).for_each(|j| {
            let coeff = self[i][j];
            let (i, j) = (i * 2, j * 2);
            (0..2).for_each(|p| (0..2).for_each(|q| {
                res[i + p][j + q] = coeff * rhs[p][q]
            }));
        }));

        res
    }
}

impl<const N: usize> Add<Self> for &Matrix<N> {
    type Output = Matrix<N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();
        (0..N).for_each(|i| (0..N).for_each(|j| {
            res[i][j] = self[i][j] + rhs[i][j];
        }));
        res
    }
}

impl<const N: usize> AddAssign<&Matrix<N>> for Matrix<N> {
    fn add_assign(&mut self, rhs: &Matrix<N>) {
        (0..N).for_each(|i| (0..N).for_each(|j| {
            self[i][j] += rhs[i][j];
        }));
    }
}

impl<const N: usize> Sub<Self> for &Matrix<N> {
    type Output = Matrix<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();
        (0..N).for_each(|i| (0..N).for_each(|j| {
            res[i][j] = self[i][j] - rhs[i][j];
        }));
        res
    }
}

impl<const N: usize> SubAssign<&Matrix<N>> for Matrix<N> {
    fn sub_assign(&mut self, rhs: &Matrix<N>) {
        (0..N).for_each(|i| (0..N).for_each(|j| {
            self[i][j] -= rhs[i][j];
        }));
    }
}

impl<const N: usize> Mul<Self> for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix::default();
        (0..N).for_each(|i| (0..N).for_each(|j| {
            res[i][j] = (0..N).map(|k| self[i][k] * rhs[k][j]).sum();
        }));
        res
    }
}

impl<const N: usize> Neg for &Matrix<N> {
    type Output = Matrix<N>;

    fn neg(self) -> Self::Output {
        let mut res = Matrix::default();
        (0..N).for_each(|i| (0..N).for_each(|j| {
            res[i][j] = -self[i][j];
        }));
        res
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
    pub const fn new_unchecked(mat: Matrix<N>) -> Self {
        Self { mat }
    }

    pub const fn eye() -> Self {
        Self::new_unchecked(Matrix::eye())
    }

    pub const fn take(self) -> Matrix<N> {
        self.mat
    }

    pub fn inv(&self) -> Self {
        // The inverse of a unitary matrix is it's conjugate transpose
        let mut res = Matrix::default();
        (0..N).for_each(|i| (0..N).for_each(|j| {
            res[i][j] = self[j][i].conj();
        }));
        Self::new_unchecked(res)
    }
}

impl<const N: usize> Mul<Self> for &UnitaryMatrix<N> {
    type Output = UnitaryMatrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        UnitaryMatrix::new_unchecked(&self.mat * &rhs.mat)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Error)]
#[error("matrix is not unitary")]
pub struct NotUnitaryError;

impl<const N: usize> TryFrom<Matrix<N>> for UnitaryMatrix<N> {
    type Error = NotUnitaryError;

    fn try_from(mat: Matrix<N>) -> Result<Self, Self::Error> {
        mat.is_unitary().then(|| Self::new_unchecked(mat)).ok_or(NotUnitaryError)
    }
}

impl From<Su2> for UnitaryMatrix<2> {
    fn from(unitary: Su2) -> Self {
        let Su2 { alpha, beta } = unitary;
        Self::new_unchecked(Matrix::new2x2(alpha, -beta.conj(), beta, alpha.conj()))
    }
}

#[derive(Clone, Debug)]
pub struct Su2 {
    alpha: c64,
    beta: c64,
}

impl Su2 {
    pub const fn new_unchecked(alpha: c64, beta: c64) -> Self {
        Self { alpha, beta }
    }

    pub fn new(alpha: c64, beta: c64) -> Option<Su2> {
        c64::is_distribution(&[alpha, beta]).then(|| Self::new_unchecked(alpha, beta))
    }

    pub fn inv(&self) -> Self {
        Su2::new_unchecked(self.alpha.conj(), -self.beta)
    }

    pub const fn alpha(&self) -> c64 {
        self.alpha
    }

    pub const fn beta(&self) -> c64 {
        self.beta
    }
}

impl From<UnitaryMatrix<2>> for Su2 {
    fn from(mat: UnitaryMatrix<2>) -> Self {
        let &[[alpha, _], [beta, _]] = mat.raw();
        Self::new_unchecked(alpha, beta)
    }
}

impl Mul<Self> for &Su2 {
    type Output = Su2;

    fn mul(self, rhs: Self) -> Self::Output {
        Su2::new_unchecked(
            self.alpha * rhs.alpha - self.beta.conj() * rhs.beta,
            self.beta * rhs.alpha + self.alpha.conj() * rhs.beta
        )
    }
}

impl TryFrom<Matrix<2>> for Su2 {
    type Error = NotUnitaryError;

    fn try_from(mat: Matrix<2>) -> Result<Self, Self::Error> {
        UnitaryMatrix::try_from(mat).map(Self::from)
    }
}