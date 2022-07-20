#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitSet {
    size: usize,
    data: Box<[u8]>,
}

#[inline]
fn word(index: usize) -> usize {
    index >> 3
}

#[inline]
fn mask(index: usize) -> u8 {
    1 << (index & 7)
}

impl BitSet {
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
    pub fn set(&mut self, index: usize, value: bool) -> Option<()> {
        (index < self.size).then(|| if value {
            self.data[word(index)] |= mask(index)
        } else {
            self.data[word(index)] &= !mask(index)
        })
    }
}