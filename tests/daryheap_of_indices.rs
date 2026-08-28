#![cfg(not(miri))]

mod priority_queue_deckey_tests;
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
    use orx_priority_queue::DaryHeapOfIndices;
    use priority_queue_deckey_tests::*;
    use priority_queue_tests::*;

    let new_heap = || DaryHeapOfIndices::<usize, f64, D>::with_index_bound(125);

    let change_key = [
        ChangeKeyMethod::Decrease,
        ChangeKeyMethod::Update,
        ChangeKeyMethod::TryDecrease,
    ];

    test_len(new_heap());
    test_is_empty(new_heap());
    test_peek(new_heap());
    test_clear(new_heap());
    test_push_pop(new_heap());
    test_push_pop_randomized(new_heap());
    test_push_then_pop(new_heap());
    test_push_then_pop_randomized(new_heap());

    test_contains(new_heap());
    test_key_of(new_heap());
    change_key
        .iter()
        .for_each(|change_key_method| test_change_key(new_heap(), *change_key_method));
    test_remove(new_heap());
    test_mixed(new_heap());

    change_key
        .iter()
        .for_each(|change_key_method| test_change_key_or_push(new_heap(), *change_key_method));
}

#[test]
fn from_iter_with_index_bound_heapifies_and_tracks_positions() {
    use orx_priority_queue::{DaryHeapOfIndices, PriorityQueue, PriorityQueueDecKey};

    let mut heap = DaryHeapOfIndices::<usize, _, 4>::from_iter_with_index_bound(
        4,
        [(0, 42), (1, 7), (2, 24), (3, 3)],
    );

    assert_eq!(heap.key_of(&0), Some(42));
    assert_eq!(heap.key_of(&1), Some(7));
    assert_eq!(heap.key_of(&2), Some(24));
    assert_eq!(heap.key_of(&3), Some(3));
    assert_eq!(heap.pop(), Some((3, 3)));
    assert_eq!(heap.pop(), Some((1, 7)));
    assert_eq!(heap.pop(), Some((2, 24)));
    assert_eq!(heap.pop(), Some((0, 42)));
}
