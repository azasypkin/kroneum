use core::fmt::{Debug, Error, Formatter};
use core::ops::Index;

const MAX_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct Array<T> {
    buffer: [T; MAX_SIZE],
    len: usize,
}

impl<T: Default + Copy> Array<T> {
    /// Creates an `Array` structure with internal initialized buffer `MAX_SIZE` size.
    pub fn new() -> Self {
        Array {
            buffer: [T::default(); MAX_SIZE],
            len: 0,
        }
    }

    /// Pushes value into `Array`. Note that if internal buffer is full, no more data will be
    /// written effectively making it read-only.
    pub fn push(&mut self, value: T) {
        if self.len < MAX_SIZE {
            self.buffer[self.len] = value;
            self.len += 1;
        }
    }

    /// Returns length of the part of internal buffer that contains values.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks whether internal buffer is empty or not.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Default + Copy> Default for Array<T> {
    fn default() -> Self {
        Array::new()
    }
}

impl<T: Copy + PartialEq> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for i in 0..self.len {
            if self[i] != other[i] {
                return false;
            }
        }

        true
    }
}

impl<T: Debug> Debug for Array<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.buffer[..32].fmt(f)
    }
}

impl<T: Copy> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }

        &self.buffer[index]
    }
}

impl<T> AsRef<[T]> for Array<T> {
    fn as_ref(&self) -> &[T] {
        &self.buffer[..self.len]
    }
}

impl<T: Default + Copy> From<&[T]> for Array<T> {
    fn from(slice: &[T]) -> Self {
        let mut array = Array::new();
        slice.iter().enumerate().for_each(|(_, n)| array.push(*n));
        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_detects_length() {
        let mut array: Array<u8> = Array::new();
        assert_eq!(array.len(), 0);

        array.push(1);
        assert_eq!(array.len(), 1);

        array.push(3);
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn correctly_detects_is_empty() {
        let mut array: Array<u8> = Array::new();
        assert_eq!(array.is_empty(), true);

        array.push(1);
        assert_eq!(array.is_empty(), false);
    }

    #[test]
    fn correctly_handles_default() {
        let array: Array<u8> = Default::default();
        assert_eq!(array.len(), 0);
        assert_eq!(array.is_empty(), true);
    }

    #[test]
    fn correctly_compares_arrays() {
        let mut array_one: Array<u8> = Array::new();
        let mut array_two: Array<u8> = Array::new();
        assert_eq!(array_one, array_two);

        array_one.push(1);
        assert_ne!(array_one, array_two);

        array_two.push(1);
        assert_eq!(array_one, array_two);

        array_one.push(2);
        assert_ne!(array_one, array_two);

        array_two.push(3);
        assert_ne!(array_one, array_two);
    }

    #[test]
    fn correctly_handles_indexing() {
        let mut array: Array<u8> = Default::default();
        array.push(11);
        array.push(22);
        array.push(33);

        assert_eq!(array[0], 11);
        assert_eq!(array[1], 22);
        assert_eq!(array[2], 33);
    }

    #[test]
    fn correctly_handles_references() {
        let mut array: Array<u8> = Default::default();
        assert_eq!(array.as_ref(), []);

        array.push(11);
        array.push(22);
        array.push(33);

        assert_eq!(array.as_ref(), [11, 22, 33]);
    }

    #[test]
    fn correctly_constructed_from_slice() {
        let array = Array::from([11, 22, 33].as_ref());
        assert_eq!(array.len(), 3);
        assert_eq!(array.as_ref(), [11, 22, 33]);
    }

    #[test]
    fn do_not_panic_if_exceeds_size() {
        let mut array: Array<u8> = Array::new();
        for value in 0..(MAX_SIZE + 5) {
            array.push(value as u8);
        }

        assert_eq!(array.len(), MAX_SIZE);
        assert_eq!(array.is_empty(), false);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn panics_if_indexing_on_empty_array() {
        let array: Array<u8> = Array::new();

        let _non_existing_value = array[0];
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn panics_if_index_is_out_of_bounds() {
        let mut array: Array<u8> = Array::new();
        array.push(11);
        array.push(22);
        array.push(33);

        let _existing_value = array[2];
        let _non_existing_value = array[3];
    }
}
