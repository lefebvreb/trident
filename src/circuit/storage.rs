use std::mem;

/// The type used for storage.
pub(crate) type Word = u32;

/// Size of a Word in bytes.
pub(crate) const WORD_SIZE: usize = mem::size_of::<Word>();

/// Alignement of a Word in bytes.
pub(crate) const WORD_ALIGN: usize = mem::align_of::<Word>();

/// Twice the size of a Word in bytes.
pub(crate) const TWO_WORD_SIZE: usize = 2 * WORD_SIZE;

/// Reads a single Word from the source.
/// Panics if the source is empty.
#[inline(always)]
fn read_word(src: &mut &[Word]) -> Word {
    // TODO: replace with <[T]>::take_first when it eventually stabilizes
    let (&first, tail) = src.split_first().unwrap();
    *src = tail;
    first
}

/// Reads a type T from the source. 
/// Panics if the size of T is not equal nor twice the size of Word, or
/// the source is empty.
#[inline(always)]
pub(crate) fn read<T>(src: &mut &[Word]) -> T {
    match mem::size_of::<T>() {
        WORD_SIZE => {
            let word = read_word(src);
            // SAFETY: we checked size.
            unsafe { mem::transmute_copy(&word) }
        }
        TWO_WORD_SIZE => {
            let words = [(); 2].map(|_| read_word(src));
            // SAFETY: we checked size.
            unsafe { mem::transmute_copy(&words) }
        }
        _ => panic!("write only supports types of size 1x or 2x the word size"),
    }
}

/// Reads a slice of T from the source. 
/// Panics if the size and align of T are not equal to that of Word, or
/// the source is empty.
#[inline(always)]
pub(crate) fn read_slice<'a, T>(src: &mut &'a [Word], n: u32) -> &'a [T] {
    if mem::size_of::<T>() != WORD_SIZE || mem::align_of::<T>() != WORD_ALIGN {
        panic!("read_slice only supports type of word size and align");
    }

    let (left, right) = src.split_at(n as usize);
    *src = right;

    // SAFETY: T and Word have the same size and align.
    unsafe { mem::transmute(left) }
}

/// Writes a single T to the destination.
/// Panics if the size of T is not equal nor twice the size of Word.
#[inline(always)]
pub(crate) fn write<T>(dest: &mut Vec<Word>, data: T) {
    match mem::size_of::<T>() {
        WORD_SIZE => {
            // SAFETY: we checked size.
            let word = unsafe { mem::transmute_copy(&data) };
            dest.push(word);
        }
        TWO_WORD_SIZE => {
            // SAFETY: we checked size.
            let words: [Word; 2] = unsafe { mem::transmute_copy(&data) };
            dest.extend(&words);
        }
        _ => panic!("write only supports types of size 1x or 2x the word size"),
    }
}

/// Writes a slice of T to the destination.
/// Panics if the size and align of T are not equal to that of Word,
#[inline(always)]
pub(crate) fn write_slice<T>(dest: &mut Vec<Word>, slice: &[T]) {
    if mem::size_of::<T>() != WORD_SIZE || mem::align_of::<T>() != WORD_ALIGN {
        panic!("write_slice only supports type of word size and align");
    }

    // SAFETY: T and Word have the same size and align.
    let slice: &[Word] = unsafe { mem::transmute(slice) };
    dest.extend(slice)
}