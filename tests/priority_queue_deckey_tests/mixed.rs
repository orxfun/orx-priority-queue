use std::collections::HashSet;

use itertools::Itertools;
use orx_priority_queue::PriorityQueueDecKey;
use rand::Rng;

pub fn test_mixed<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    const INITIAL_LEN: usize = 10;
    const LEN: usize = 125;
    let mut rng = rand::thread_rng();

    pq.clear();
    assert!(pq.is_empty());

    for _ in 0..INITIAL_LEN {
        let push = rng.gen_range(0..LEN);
        let priority = rng.gen();
        if !pq.contains(&push) {
            pq.push(push, priority);
        }
    }
    for _ in 0..100 {
        let num_drop = if pq.is_empty() {
            0
        } else {
            rng.gen_range(0..5)
        };
        let enqueued = pq.as_slice().iter().map(|x| x.0).collect_vec();
        let mut to_drop = HashSet::new();
        for _ in 0..num_drop {
            let ind = rng.gen_range(0..enqueued.len());
            to_drop.insert(enqueued[ind]);
        }

        let mut to_push = vec![];
        for _ in 0..to_drop.len() {
            let num_push = rng.gen_range(0..5);
            let mut to_push_for = vec![];
            for _ in 0..num_push {
                let ind = rng.gen_range(0..LEN);
                if !pq.contains(&ind) {
                    to_push_for.push(ind);
                }
            }
            to_push.push(to_push_for);
        }

        for (drop, to_push_for) in to_drop.iter().zip(to_push) {
            pq.remove(drop);

            for push in &to_push_for {
                if pq.len() == LEN {
                    break;
                }
                let priority = rng.gen();
                if !pq.contains(push) {
                    pq.push(*push, priority);
                }
            }
        }

        if !pq.is_empty() {
            pq.pop();
        }
    }

    while !pq.is_empty() {
        pq.pop();
    }
}
