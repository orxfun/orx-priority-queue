mod priority_queue_deckey_tests;
mod priority_queue_tests;

use orx_priority_queue::DaryHeapOfIndices;
use priority_queue_deckey_tests::*;
use priority_queue_tests::*;

const D: usize = 2;

fn new_heap() -> DaryHeapOfIndices<usize, f64, D> {
    DaryHeapOfIndices::with_upper_limit(125)
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
fn as_slice() {
    test_as_slice(new_heap())
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

#[test]
fn contains() {
    test_contains(new_heap());
}

#[test]
fn key_of() {
    test_key_of(new_heap());
}

#[test]
fn decrease_key() {
    test_change_key(new_heap(), ChangeKeyMethod::Decrease);
}

#[test]
fn update_key() {
    test_change_key(new_heap(), ChangeKeyMethod::Update);
}

#[test]
fn try_decrease_key() {
    test_change_key(new_heap(), ChangeKeyMethod::TryDecrease);
}

#[test]
fn remove() {
    test_remove(new_heap());
}

#[test]
fn mixed() {
    test_mixed(new_heap());
}

#[test]
fn decrease_key_or_push() {
    test_change_key_or_push(new_heap(), ChangeKeyMethod::Decrease);
}

#[test]
fn update_key_or_push() {
    test_change_key_or_push(new_heap(), ChangeKeyMethod::Update);
}

#[test]
fn try_decrease_key_or_push() {
    test_change_key_or_push(new_heap(), ChangeKeyMethod::TryDecrease);
}
