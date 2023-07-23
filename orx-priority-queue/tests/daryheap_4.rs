mod priority_queue_tests;

use orx_priority_queue::DaryHeap;
use priority_queue_tests::*;

const D: usize = 4;

fn new_heap() -> DaryHeap<usize, f64, D> {
    DaryHeap::default()
}

#[test]
fn len() {
    test_len(new_heap())
}

#[test]
fn is_empty() {
    test_is_empty(new_heap())
}

#[test]
fn peek() {
    test_peek(new_heap())
}

#[test]
fn clear() {
    test_clear(new_heap())
}

#[test]
fn push_pop() {
    test_push_pop(new_heap())
}

#[test]
fn push_pop_randomized() {
    test_push_pop_randomized(new_heap())
}

#[test]
fn push_then_pop() {
    test_push_then_pop(new_heap())
}
#[test]
fn push_then_pop_randomized() {
    test_push_then_pop_randomized(new_heap())
}
