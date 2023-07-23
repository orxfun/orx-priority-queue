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
