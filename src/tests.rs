use crate::ringbuf::*;

#[test]
fn test_ring_buffer() {
    let mut rb = RingBuffer::with_capacity(3);
    rb.push(1);
    rb.push(2);
    assert!(!rb.is_full());
    assert_eq!(rb.len(), 2);
    assert_eq!(rb.capacity(), 3);
    rb.push_ignore(3);
    rb.push_ignore(4);
    let sum = rb.iter().sum::<i32>();
    assert_eq!(sum, 9);
    assert!(rb.is_full());
    assert_eq!(rb.len(), 3);
    assert_eq!(rb.capacity(), 3);
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), Some(4));
    assert_eq!(rb.pop(), None);
    assert!(rb.is_empty());
    assert_eq!(rb.capacity(), 3);
    assert_eq!(rb.len(), 0);
}

#[test]
fn test_ring_buffer_push() {
    let mut rb = RingBuffer::with_capacity(8);
    for i in 0..128 {
        rb.push_ignore(i);
    }
    // Make sure that RB iterator ends.
    for (i, &v) in (120..1024).zip(&rb) {
        assert_eq!(i, v);
    }
    assert!(!rb.push(129));
    assert_eq!(rb.len(), rb.capacity());
    assert!(!rb.is_empty());
}

#[test]
fn test_ring_buffer_pop() {
    let mut rb = RingBuffer::with_capacity(3);
    for i in 1..=6 {
        rb.push(i);
    }
    assert_eq!(rb.pop(), Some(1));
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), None);
}
