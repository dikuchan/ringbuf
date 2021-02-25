#![allow(unused_must_use)]

use crate::*;

#[test]
fn test_ring_buffer() {
    let mut rb = RingBuffer::with_capacity(6);

    assert_eq!(rb.is_empty(), true);
    assert_eq!(rb.is_full(), false);

    for v in 1..=10 {
        rb.push_ignore(v);
    }

    assert_eq!(rb.capacity(), 6);
    assert_eq!(rb.len(), 6);
    assert_eq!(rb.is_full(), true);

    for (&i, &v) in [7, 8, 9, 10, 5, 6].iter().zip(&rb) {
        assert_eq!(i, v);
    }

    while let Some(_) = rb.pop() {}

    for v in 13..=37 {
        rb.push(v);
    }

    for (i, v) in (13..=1000).zip(rb) {
        assert_eq!(i, v);
    }
}
