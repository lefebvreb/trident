use std::mem;

/// The length of usizes in words.
pub(crate) const USIZE_LEN: usize = mem::size_of::<usize>() / mem::size_of::<u32>();

/// Reads a single word (u32) from the source.
#[inline(always)]
pub(crate) fn read(src: &mut &[u32]) -> u32 {
    // TODO: replace with <[T]>::take_first when it eventually stabilizes
    let (&first, tail) = src.split_first().unwrap();
    *src = tail;
    first
}

/// Reads a slice of n words (&[u32]) from the source.
#[inline(always)]
pub(crate) fn reads<'a>(src: &mut &'a [u32], n: u32) -> &'a [u32] {
    // TODO: replace with <[T]>::take when it eventually stabilizes
    let (left, right) = src.split_at(n as usize);
    *src = right;
    left
}

/// Writes a single word (u32) to the destination.
#[inline(always)]
pub(crate) fn write<'a>(dest: &mut Vec<u32>, word: u32) {
    dest.push(word);
}

/// Writes a slice of n words (&[u32]) to the destination.
#[inline(always)]
pub(crate) fn writes<'a>(dest: &mut Vec<u32>, words: &[u32]) {
    dest.extend(words);
}