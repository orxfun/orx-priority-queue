mod priority_queue_tests;

use orx_priority_queue::DaryHeap;
use priority_queue_tests::*;

const D: usize = 4;

fn empty_heap() -> DaryHeap<usize, f64, D> {
    DaryHeap::default()
}

#[test]
fn len() {
    test_len(empty_heap())
}

#[test]
fn is_empty() {
    test_is_empty(empty_heap())
}

#[test]
fn peek() {
    test_peek(empty_heap())
}

#[test]
fn clear() {
    test_clear(empty_heap())
}

#[test]
fn push_pop() {
    test_push_pop(empty_heap())
}

#[test]
fn push_pop_randomized() {
    test_push_pop_randomized(empty_heap())
}
