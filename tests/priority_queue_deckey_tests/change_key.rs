use itertools::Itertools;
use orx_priority_queue::{PriorityQueueDecKey, ResTryDecreaseKey, ResUpdateKey};
use rand::prelude::*;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum ChangeKeyMethod {
    Decrease,
    Update,
    TryDecrease,
}

pub fn test_change_key<P>(mut pq: P, change_key: ChangeKeyMethod)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    const LEN: usize = 100;

    pq.clear();
    assert!(pq.is_empty());

    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    // push 100
    for node in 0..LEN {
        let priority = rng.gen();
        pq.push(node, priority);
        vec.push((node, priority));
    }

    // change keys 100 times
    for _ in 0..LEN {
        let node = rng.gen_range(0..LEN);
        let old_key = vec[node].1;
        assert_eq!(Some(old_key), pq.key_of(&node));

        let new_key = rng.gen::<f64>()
            * match change_key {
                ChangeKeyMethod::Decrease => old_key,
                _ => 1.0,
            };

        match change_key {
            ChangeKeyMethod::Decrease => {
                pq.decrease_key(&node, new_key);
                vec[node] = (node, new_key);
            }
            ChangeKeyMethod::Update => {
                let res_updkey = pq.update_key(&node, new_key);
                assert_eq!(
                    new_key < old_key,
                    matches!(res_updkey, ResUpdateKey::Decreased)
                );
                vec[node] = (node, new_key);
            }
            ChangeKeyMethod::TryDecrease => {
                let res_try_deckey = pq.try_decrease_key(&node, new_key);
                assert_eq!(
                    new_key < old_key,
                    matches!(res_try_deckey, ResTryDecreaseKey::Decreased)
                );
                if matches!(res_try_deckey, ResTryDecreaseKey::Decreased) {
                    vec[node] = (node, new_key);
                }
            }
        }

        assert_eq!(Some(vec[node].1), pq.key_of(&node));
    }

    // pop in correct order
    let vec_sorted = vec
        .into_iter()
        .sorted_by(|x, y| {
            if x.1 <= y.1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .collect_vec();
    for vec_popped in vec_sorted {
        let pq_popped = pq.pop();
        assert_eq!(Some(vec_popped), pq_popped);
    }
    assert!(pq.is_empty());
}
