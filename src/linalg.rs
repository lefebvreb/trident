use std::ops::Deref;

type c32 = ();

pub struct Matrix<const N: usize>  {
    data: [[c32; N]; N],
}

pub struct UnitaryMatrix<const N: usize> {
    mat: Matrix<N>
}

impl<const N: usize> Deref for UnitaryMatrix<N> {
    type Target = Matrix<N>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.mat
    }
}