use crate::*;

#[test]
fn test_ring_buffer_general() {
    let mut rb = RingBuffer::with_capacity(3);
    rb.push("rin");
    rb.push("gb");

    assert!(!rb.is_full());
    assert_eq!(rb.len(), 2);
    assert_eq!(rb.capacity(), 3);

    rb.push_ignore("uff");
    rb.push_ignore("er");
    let name: String = rb.iter().flat_map(|s| s.chars()).collect();

    assert_ne!(name, "ringbuffer".to_owned());
    assert!(rb.is_full());
    assert_eq!(rb.len(), rb.capacity());

    assert_eq!(rb.pop(), Some("gb"));
    assert_eq!(rb.pop(), Some("uff"));
    assert_eq!(rb.pop(), Some("er"));
    assert_eq!(rb.pop(), None);

    assert!(rb.is_empty());
    assert_eq!(rb.capacity(), 3);
    assert_eq!(rb.len(), 0);
}

#[test]
fn test_ring_buffer_push_pop() {
    let mut rb: RingBuffer<_> = (1..=5).collect();
    rb.push_ignore(1);
    rb.push_ignore(2);

    assert_eq!(rb.head, 2);
    assert_eq!(rb.head, rb.tail);

    rb.pop();
    assert!(rb.push(3));
    assert!(rb.is_full());
    assert_eq!(rb.head, rb.tail);

    assert_eq!(rb.pop(), Some(4));
    assert_eq!(rb.pop(), Some(5));
    assert_eq!(rb.pop(), Some(1));
    assert_eq!(rb.pop(), Some(2));
    assert_eq!(rb.pop(), Some(3));

    assert_eq!(rb.head, rb.tail);
    assert!(rb.is_empty());
}

#[test]
fn test_ring_buffer_iterator() {
    let mut rb: RingBuffer<_> = (1..=6).collect();

    assert!(!rb.push(7));
    assert_eq!(rb.len(), rb.capacity());
    assert!(!rb.is_empty());

    // Check if iterator ends.
    for (i, v) in (1..1024).zip(rb) {
        assert_eq!(i, v);
    }

    let mut rb = RingBuffer::with_capacity(8);
    for i in 0..128 {
        rb.push_ignore(i);
    }
    for (i, &v) in (120..1024).zip(&rb) {
        assert_eq!(i, v);
    }

    // Check slice iterator.
    let mut rb: RingBuffer<_> = (1..=8).collect();
    rb.push_ignore(9);
    rb.push_ignore(10);
    for v in rb.iter() { let _ = v; }

    assert_eq!(rb.len(), 8);
    assert_eq!(rb.pop(), Some(3));
    assert_eq!(rb.pop(), Some(4));
}
