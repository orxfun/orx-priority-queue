#![cfg(not(miri))]

mod priority_queue_tests;

#[test]
fn test_dary_forall() {
    test_dary_for::<2>();
    test_dary_for::<3>();
    test_dary_for::<4>();
    test_dary_for::<7>();
    test_dary_for::<8>();
    test_dary_for::<13>();
    test_dary_for::<16>();
    test_dary_for::<32>();
    test_dary_for::<64>();
}

fn test_dary_for<const D: usize>() {
    use orx_priority_queue::DaryHeap;
    use priority_queue_tests::*;

    let new_heap = DaryHeap::<usize, f64, D>::default;

    test_len(new_heap());
    test_is_empty(new_heap());
    test_as_slice(new_heap());
    test_peek(new_heap());
    test_clear(new_heap());
    test_push_pop(new_heap());
    test_push_pop_randomized(new_heap());
    test_push_then_pop(new_heap());
    test_push_then_pop_randomized(new_heap());
}

#[test]
fn from_iter_heapifies() {
    use orx_priority_queue::PriorityQueue;

    let mut heap = [(0, 42), (1, 7), (2, 24), (3, 3), (4, 99), (5, 11)]
        .into_iter()
        .collect::<orx_priority_queue::QuaternaryHeap<_, _>>();

    assert_eq!(heap.len(), 6);
    assert_eq!(heap.pop(), Some((3, 3)));
    assert_eq!(heap.pop(), Some((1, 7)));
    assert_eq!(heap.pop(), Some((5, 11)));
    assert_eq!(heap.pop(), Some((2, 24)));
    assert_eq!(heap.pop(), Some((0, 42)));
    assert_eq!(heap.pop(), Some((4, 99)));
    assert_eq!(heap.pop(), None);
}
