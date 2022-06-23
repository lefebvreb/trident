#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitVec {
    size: usize,
    data: Box<[u8]>,
}

#[inline(always)]
fn word(index: usize) -> usize {
    index >> 3
}

#[inline(always)]
fn mask(index: usize) -> u8 {
    1 << (index & 7)
}

impl BitVec {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![0; word(size)].into_boxed_slice(),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        (index < self.size).then(|| self.data[word(index)] & mask(index) == 1)
    }

    #[inline]
    pub fn set(&mut self, index: usize) -> Option<()> {
        (index < self.size).then(|| self.data[word(index)] |= mask(index))
    }
}