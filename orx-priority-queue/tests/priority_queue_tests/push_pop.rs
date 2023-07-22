use std::cmp::Ordering;

use itertools::Itertools;
use orx_priority_queue::PriorityQueue;
use rand::prelude::*;

pub fn test_push_pop<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    const N: usize = 3;
    const M: usize = 3;

    pq.clear();
    assert_eq!(0, pq.len());

    // push with ascending priorities
    for i in 0..N {
        pq.push(i, i as f64);
    }
    assert_eq!(N, pq.len());

    // push with descending priorities
    for i in 0..M {
        pq.push(N + i, (N + M - i) as f64);
    }
    assert_eq!(N + M, pq.len());

    // popped in correct order
    for i in 0..N {
        assert_eq!(i, pq.pop_node().unwrap());
    }
    assert_eq!(M, pq.len());

    for i in 0..M {
        assert_eq!(N + M - 1 - i, pq.pop_node().unwrap());
    }
    assert!(pq.is_empty());
}

pub fn test_push_pop_randomized<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    let mut rng = rand::thread_rng();

    pq.clear();
    assert!(pq.is_empty());

    let mut vec = Vec::new();

    // push 100 -> 0, ..., 99
    for node in 0..100 {
        let priority = rng.gen();
        pq.push(node, priority);
        vec.push((node, priority));
    }
    assert_eq!(100, pq.len());

    // pop 60
    let mut vec_popped60 = vec
        .into_iter()
        .sorted_by(|x, y| {
            if x.1 <= y.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .collect_vec();
    let mut vec_remaining40 = vec_popped60.split_off(60);
    assert_eq!(40, vec_remaining40.len());
    assert_eq!(60, vec_popped60.len());
    for vec_popped in vec_popped60 {
        let pq_popped = pq.pop();
        assert_eq!(Some(vec_popped), pq_popped);
    }
    assert_eq!(40, pq.len());

    // push 25 -> 100, ..., 124
    for node in 100..125 {
        let priority = rng.gen();
        pq.push(node, priority);
        vec_remaining40.push((node, priority));
    }
    let vec_remaining65 = vec_remaining40
        .into_iter()
        .sorted_by(|x, y| {
            if x.1 <= y.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .collect_vec();
    assert_eq!(65, vec_remaining65.len());
    assert_eq!(65, pq.len());

    // pop 65
    for vec_popped in vec_remaining65 {
        let pq_popped = pq.pop();
        assert_eq!(Some(vec_popped), pq_popped);
    }

    assert!(pq.is_empty());
}
