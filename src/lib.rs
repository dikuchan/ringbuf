use std::iter::FromIterator;

#[cfg(test)]
mod tests;

pub struct RingBufferError;

/// The usage of unsafe features (e.g. `MaybeUninit`) is restricted.
/// Thus, the following solution requires a type to implement the `Default` trait.
pub struct RingBuffer<T> {
    data: Vec<T>,
    head: usize,
    tail: usize,
}

impl<T: Default> RingBuffer<T> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: (0..n).map(|_| T::default()).collect(),
            head: 0,
            tail: 0,
        }
    }

    fn mask(&self, v: usize) -> usize {
        v % self.capacity()
    }

    /// Appends an element to the buffer if the buffer is not full.
    /// Reader pointer is shifted at the oldest element.
    pub fn push(&mut self, v: T) -> Result<(), RingBufferError> {
        if self.is_full() { return Err(RingBufferError); }

        self.push_ignore(v);

        Ok(())
    }

    /// Appends an element to the buffer even if the buffer is full.
    pub fn push_ignore(&mut self, v: T) {
        self.head += 1;
        let index = self.mask(self.head);
        self.data[index] = v;
    }

    /// Removes the last added element from the buffer and returns it.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }

        self.tail += 1;
        let index = self.mask(self.tail);
        let result = std::mem::take(&mut self.data[index]);

        Some(result)
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.head = 0;
        self.tail = 0;
    }

    /// Returns `true` if the buffer contains no elements.
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Returns `true` if `len` is equal to `capacity`.
    pub fn is_full(&self) -> bool { 
        self.len() == self.capacity() 
    }

    /// Returns the number of elements the buffer holds.
    pub fn len(&self) -> usize {
        let delta = self.head - self.tail;
        if delta >= self.capacity() {
            self.capacity()
        } else {
            self.mask(delta)
        }
    }

    /// Returns the number of elements the buffer is able to hold.
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            buffer: &self,
            index: self.tail,
            traversed: false,
        }
    }

    /// Returns a consuming iterator.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { buffer: self }
    }
}

impl<T: Default> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter { self.into_iter() }
}

/// Iterator for the values of `T`.
pub struct IntoIter<T: Default> {
    buffer: RingBuffer<T>,
}

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { self.buffer.pop() }
}

impl<'a, T: Default> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// Iterator for the values of `&T`.
pub struct Iter<'a, T: Default> {
    buffer: &'a RingBuffer<T>,
    index: usize,
    traversed: bool,
}

impl<'a, T: Default> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.mask(self.index) == self.buffer.mask(self.buffer.head) {
            if self.traversed { return None; } else { self.traversed = true; }
        }
        self.index += 1;
        let i = self.buffer.mask(self.index);
        let result = &self.buffer.data[i];

        Some(result)
    }
}

impl<T: Default> FromIterator<T> for RingBuffer<T> {
    /// Conversion from an `Iterator`.
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut data = Vec::<T>::new();

        for i in iter { data.push(i); }
        data.shrink_to_fit();

        RingBuffer {
            head: 0,
            data,
            tail: 0,
        }
    }
}
