use std::iter::FromIterator;

#[cfg(test)]
mod tests;

/// The usage of unsafe features (e.g. `MaybeUninit`) is restricted.
/// Thus, this solution requires a type to implement the `Default` trait.
pub struct RingBuffer<T> {
    data: Vec<T>,
    head: usize,
    tail: usize,
    full: bool,
}

impl<T: Default + Clone> RingBuffer<T> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: vec![T::default(); n],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    /// Appends an element to the buffer if the buffer is not full.
    /// Returns `true` if the element was added.
    pub fn push(&mut self, v: T) -> bool {
        if self.is_full() { return false; }

        self.data[self.head] = v;
        self.head = (self.head + 1) % self.data.capacity();
        self.full = self.head == self.tail;

        true
    }

    /// Appends an element to the buffer even if the buffer is full.
    /// Reader pointer is shifted at the oldest element.
    pub fn push_ignore(&mut self, v: T) {
        self.data[self.head] = v;
        if self.full { self.tail = (self.tail + 1) % self.data.capacity(); }
        self.head = (self.head + 1) % self.data.capacity();
        self.full = self.head == self.tail;
    }

    /// Removes the last added element from the buffer and returns it.
    /// Returns `false` if the buffer is empty and the element was not removed.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() { return None; }

        let result = std::mem::take(&mut self.data[self.tail]);
        self.full = false;
        self.tail = (self.tail + 1) % self.data.capacity();

        Some(result)
    }

    /// Clears the buffer, removing all values.
    pub fn clear(&mut self) {
        self.data.clear();
        self.head = 0;
        self.tail = 0;
        self.full = false;
    }

    /// Returns `true` if the buffer contains no elements.
    pub fn is_empty(&self) -> bool {
        !self.full && (self.head == self.tail)
    }

    /// Returns `true` if `len` is equal to `capacity`.
    pub fn is_full(&self) -> bool { self.full }

    /// Returns the number of elements the buffer holds.
    pub fn len(&self) -> usize {
        if self.full { return self.data.capacity(); }
        if self.head >= self.tail {
            self.head - self.tail
        } else {
            self.data.capacity() + self.head - self.tail
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

impl<T: Default + Clone> IntoIterator for RingBuffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter { self.into_iter() }
}

/// Iterator for the values of `T`.
pub struct IntoIter<T: Default + Clone> {
    buffer: RingBuffer<T>,
}

impl<T: Default + Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { self.buffer.pop() }
}

impl<'a, T: Default + Clone> IntoIterator for &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// Iterator for the values of `&T`.
pub struct Iter<'a, T: Default + Clone> {
    buffer: &'a RingBuffer<T>,
    index: usize,
    traversed: bool,
}

impl<'a, T: Default + Clone> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.buffer.head {
            if self.traversed { return None; } else { self.traversed = true; }
        }
        let result = &self.buffer.data[self.index];
        self.index = (self.index + 1) % self.buffer.data.capacity();

        Some(result)
    }
}

impl<T: Default + Clone> FromIterator<T> for RingBuffer<T> {
    /// Conversion from an `Iterator`.
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut data = Vec::<T>::new();

        for i in iter { data.push(i); }
        data.shrink_to_fit();

        RingBuffer {
            head: 0,
            data,
            tail: 0,
            full: true,
        }
    }
}
