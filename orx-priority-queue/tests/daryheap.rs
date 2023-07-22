mod priority_queue_tests;

use orx_priority_queue::DaryHeap;
use priority_queue_tests::*;

#[test]
fn test_dary_forall() {
    test_dary_for::<2>();
    test_dary_for::<3>();
    test_dary_for::<4>();
    test_dary_for::<7>();
    test_dary_for::<8>();
    test_dary_for::<13>();
    test_dary_for::<16>();
}

fn test_dary_for<const D: usize>() {
    let empty_heap = DaryHeap::<usize, f64, D>::default;

    test_len(empty_heap());
    test_is_empty(empty_heap());
    test_peek(empty_heap());
    test_clear(empty_heap());
    test_push_pop(empty_heap());
    test_push_pop_randomized(empty_heap())
}
