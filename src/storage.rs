//! This module exposes functions to write/read various types from vectors/slices
//! of Word. This is intended. 

use std::mem;

/// The type used for storage.
type Word = u32;

/// Checks that T and Word have the same size and align
fn assert_transparent<T>() {
    assert_eq!(mem::size_of::<T>(), mem::size_of::<Word>(), "T and Word must be the same size");
    assert_eq!(mem::align_of::<T>(), mem::align_of::<Word>(), "T and Word must be the same align");
}

fn size_multiple<T>() -> usize {
    assert_ne!(mem::size_of::<Word>(), 0, "Word can't be a ZST");
    assert_eq!(mem::size_of::<T>() % mem::size_of::<Word>(), 0, "T's size must be a multiple of Word's");
    mem::size_of::<T>() / mem::size_of::<Word>()
}

/// Reads a single Word from the source.
/// Panics if the source is empty.
fn read_word(src: &mut &[Word]) -> Word {
    // TODO: replace with <[T]>::take_first when it eventually stabilizes
    let (&first, tail) = src.split_first().unwrap();
    *src = tail;
    first
}

/// Reads a type T from the source. 
/// Panics if the size of T is not equal nor twice the size of Word, or
/// the source is empty.
pub(crate) fn read<T>(src: &mut &[Word]) -> T {
    match size_multiple::<T>() {
        1 => {
            let word = read_word(src);
            // SAFETY: we checked size.
            unsafe { mem::transmute_copy(&word) }
        }
        2 => {
            let words = [(); 2].map(|_| read_word(src));
            // SAFETY: we checked size.
            unsafe { mem::transmute_copy(&words) }
        }
        _ => panic!("read only supports types of size 1x or 2x the word size"),
    }
}

/// Reads a slice of T from the source. 
/// Panics if the size and align of T are not equal to that of Word, or
/// the source is empty.
/// 
/// For robustness this function should only be used when T is of `repr(transparent)` with
/// Word.
pub(crate) fn read_slice<'src, T>(src: &mut &'src [Word], len: u32) -> &'src [T] {
    assert_transparent::<T>();

    let (left, right) = src.split_at(len as usize);
    *src = right;

    // SAFETY: T and Word have the same size and align.
    unsafe { mem::transmute(left) }
}

/// Writes a single T to the destination.
/// Panics if the size of T is not equal nor twice the size of Word.
pub(crate) fn write<T>(dest: &mut Vec<Word>, data: T) {
    match size_multiple::<T>() {
        1 => {
            // SAFETY: we checked size.
            let word = unsafe { mem::transmute_copy(&data) };
            dest.push(word);
        }
        2 => {
            // SAFETY: we checked size.
            let words: [Word; 2] = unsafe { mem::transmute_copy(&data) };
            dest.extend(&words);
        }
        _ => panic!("write only supports types of size 1x or 2x the word size"),
    }
}

/// Writes a slice of T to the destination.
/// Panics if the size and align of T are not equal to that of Word.
/// 
/// For robustness this function should only be used when T is of `repr(transparent)` with
/// Word.
pub(crate) fn write_slice<T>(dest: &mut Vec<Word>, slice: &[T]) {
    assert_transparent::<T>();

    // SAFETY: T and Word have the same size and align.
    let slice: &[Word] = unsafe { mem::transmute(slice) };
    dest.extend(slice)
}