use itertools::Itertools;
use orx_priority_queue::{DaryHeap, PriorityQueue};
use rand::prelude::*;
use std::cmp::Ordering;

#[allow(clippy::needless_lifetimes)]
fn order<'a, 'b>(node_key_1: &'a &(usize, f64), node_key_2: &'b &(usize, f64)) -> Ordering {
    if node_key_1.1 <= node_key_2.1 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[allow(dead_code)]
pub fn test_as_slice<const D: usize>(mut pq: DaryHeap<usize, f64, D>) {
    const N: usize = 50;

    // fill it up
    pq.clear();
    assert!(pq.is_empty());
    let mut vec = vec![];

    let mut rng = rand::thread_rng();
    for node in 0..N {
        let priority = rng.gen();
        pq.push(node, priority);
        vec.push((node, priority));
    }

    // check equality
    assert_eq!(
        vec.iter().sorted_by(order).collect_vec(),
        pq.as_slice().iter().sorted_by(order).collect_vec()
    );

    // pop half of it
    for _ in 0..N / 2 {
        pq.pop();
        let argmin = vec
            .iter()
            .enumerate()
            .min_by(|x, y| order(&x.1, &y.1))
            .unwrap()
            .0;
        vec.remove(argmin);
    }

    // check equality
    assert_eq!(
        vec.iter().sorted_by(order).collect_vec(),
        pq.as_slice().iter().sorted_by(order).collect_vec()
    );
}
