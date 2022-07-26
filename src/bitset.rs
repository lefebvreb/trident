#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BitSet {
    size: usize,
    data: Box<[u8]>,
}

/// Returns the index of the word (byte) that contains the `index`th bit.
fn word(index: usize) -> usize {
    index >> 3
}

/// Returns a mask of the position of the `index`th bit in it's containing word (byte).
fn mask(index: usize) -> u8 {
    1 << (index & 7)
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![0; word(size)].into_boxed_slice(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        (index < self.size).then(|| self.data[word(index)] & mask(index) == 1)
    }

    pub fn set(&mut self, index: usize, value: bool) -> Option<()> {
        (index < self.size).then(|| if value {
            self.data[word(index)] |= mask(index)
        } else {
            self.data[word(index)] &= !mask(index)
        })
    }
}