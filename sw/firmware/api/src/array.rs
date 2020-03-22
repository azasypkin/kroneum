use core::fmt::{Debug, Error, Formatter};
use core::ops::{Index, IndexMut};

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
        if self.len < self.buffer.len() {
            self.buffer[self.len] = value;
            self.len += 1;
        }
    }

    /// Adds the specified value into the beginning of the `Array`. Note that if internal buffer is
    /// full, no more data will be written effectively making it read-only.
    pub fn unshift(&mut self, value: T) {
        if self.len < self.buffer.len() {
            let mut buffer = [T::default(); MAX_SIZE];
            buffer[0] = value;
            buffer[1..self.len + 1].copy_from_slice(&self.buffer[..self.len]);

            self.buffer = buffer;
            self.len += 1;
        }
    }

    pub fn shift(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let mut buffer = [T::default(); MAX_SIZE];
        buffer[..self.len - 1].copy_from_slice(&self.buffer[1..self.len]);

        let shifted_value = self[0];
        self.buffer = buffer;
        self.len -= 1;

        Some(shifted_value)
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

impl<T: Copy> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.len, index
            );
        }

        &mut self.buffer[index]
    }
}

impl<T> AsRef<[T]> for Array<T> {
    fn as_ref(&self) -> &[T] {
        &self.buffer[..self.len]
    }
}

impl<T> AsMut<[T]> for Array<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.buffer[..self.len].as_mut()
    }
}

impl<'a, T, I: 'a + Copy + Default> From<T> for Array<I>
where
    T: IntoIterator<Item = &'a I>,
{
    fn from(slice: T) -> Self {
        let mut array = Array::new();
        slice.into_iter().for_each(|n| array.push(*n));
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
        let array = Array::from(&[11, 22, 33]);
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
    fn correctly_copied_into_buffer() {
        let array = Array::<u8>::from(&[1, 2, 3, 3, 2, 1]);

        let mut buffer = [0xFFu8; 10];
        buffer[1..array.len() + 1].copy_from_slice(array.as_ref());

        assert_eq!(buffer, [0xFF, 1, 2, 3, 3, 2, 1, 0xFF, 0xFF, 0xFF]);
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

    #[test]
    fn correctly_handles_push() {
        let mut array: Array<u8> = Array::from(&[0, 1]);
        assert_eq!(array.is_empty(), false);
        assert_eq!(array.len(), 2);

        // Push new values.
        for i in 2..MAX_SIZE {
            array.push(i as u8);
            assert_eq!(array.len(), i + 1);
        }

        // We cannot push anymore.
        assert_eq!(array.len(), MAX_SIZE);
        array.push(MAX_SIZE as u8);
        assert_eq!(array.len(), MAX_SIZE);

        // Check all values.
        for i in 0..MAX_SIZE {
            assert_eq!(array[i], i as u8);
        }
    }

    #[test]
    fn correctly_handles_unshift() {
        let mut array: Array<u8> = Array::from(&[62, 63]);
        assert_eq!(array.is_empty(), false);
        assert_eq!(array.len(), 2);

        // Unshift new values.
        for i in (0..MAX_SIZE - 2).rev() {
            array.unshift(i as u8);
            assert_eq!(array.len(), MAX_SIZE - i);
        }

        // We cannot unshift anymore.
        assert_eq!(array.len(), MAX_SIZE);
        array.unshift(MAX_SIZE as u8);
        assert_eq!(array.len(), MAX_SIZE);

        // Check all values.
        for i in 0..MAX_SIZE {
            assert_eq!(array[i], i as u8);
        }
    }
}
