use super::ChangeKeyMethod;
use itertools::Itertools;
use orx_priority_queue::{PriorityQueueDecKey, ResTryDecreaseKeyOrPush, ResUpdateKeyOrPush};
use rand::prelude::*;
use std::cmp::Ordering;

pub fn test_change_key_or_push<P>(mut pq: P, change_key: ChangeKeyMethod)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    fn push_at_first_pass(i: usize) -> bool {
        i % 3 != 0
    }

    pq.clear();
    assert!(pq.is_empty());

    pq.clear();
    assert!(pq.is_empty());

    let mut rng = rand::thread_rng();
    let mut vec = (0..100).map(|_| None).collect_vec();

    // push 100
    for (node, vec_elem) in vec.iter_mut().enumerate() {
        if push_at_first_pass(node) {
            let priority = rng.gen();
            pq.push(node, priority);
            *vec_elem = Some(priority);
        }
    }

    // change keys or push 300 times
    for _ in 0..300 {
        let node = rng.gen_range(0..100);
        let old_key = vec[node];
        assert_eq!(old_key, pq.key_of(&node));

        let new_key = rng.gen::<f64>()
            * match change_key {
                ChangeKeyMethod::Decrease => old_key.unwrap_or(1.0),
                _ => 1.0,
            };

        match change_key {
            ChangeKeyMethod::Decrease => {
                pq.decrease_key_or_push(&node, new_key);
                vec[node] = Some(new_key);
            }
            ChangeKeyMethod::Update => {
                let res_updkey_push = pq.update_key_or_push(&node, new_key);
                assert_eq!(
                    old_key.map(|old_key| new_key < old_key).unwrap_or(false),
                    matches!(res_updkey_push, ResUpdateKeyOrPush::Decreased)
                );
                vec[node] = Some(new_key);
            }
            ChangeKeyMethod::TryDecrease => {
                let res_try_deckey_push = pq.try_decrease_key_or_push(&node, new_key);
                assert_eq!(
                    old_key.map(|old_key| new_key < old_key).unwrap_or(false),
                    matches!(res_try_deckey_push, ResTryDecreaseKeyOrPush::Decreased)
                );
                if old_key.is_none() || new_key < old_key.unwrap() {
                    vec[node] = Some(new_key);
                }
            }
        }

        assert_eq!(vec[node], pq.key_of(&node));
    }

    // pop in correct order
    let vec_sorted = vec
        .into_iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .sorted_by(|x, y| {
            if x.1 <= y.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .map(|x| (x.0, x.1.unwrap()))
        .collect_vec();
    for vec_popped in vec_sorted {
        let pq_popped = pq.pop();
        assert_eq!(Some(vec_popped), pq_popped);
    }
    assert!(pq.is_empty());
}
