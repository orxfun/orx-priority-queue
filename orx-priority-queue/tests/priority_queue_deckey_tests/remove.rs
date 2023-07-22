use orx_priority_queue::PriorityQueueDecKey;
use rand::prelude::*;
use std::collections::HashSet;

pub fn test_remove<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    const LEN: usize = 125;

    pq.clear();
    assert!(pq.is_empty());

    let mut rng = rand::thread_rng();

    let mut vec = Vec::new();
    for node in 0..LEN {
        let priority = rng.gen();
        pq.push(node, priority);
        vec.push(priority);
    }
    assert_eq!(LEN, pq.len());
    assert_eq!(LEN, vec.len());

    // remove randomly ~60%
    let mut removed = HashSet::new();
    for (node_to_rmv, key) in vec.iter().enumerate() {
        let do_remove = rng.gen::<f64>() < 0.6;
        if do_remove {
            let key_removed = pq.remove(&node_to_rmv);
            assert_eq!(key, &key_removed);
            removed.insert(node_to_rmv);

            assert_eq!(LEN, pq.len() + removed.len());
            assert!(!pq.contains(&node_to_rmv));
            assert_eq!(None, pq.key_of(&node_to_rmv));
        }
    }
    assert_eq!(LEN, pq.len() + removed.len());

    // pop remaining
    while let Some((node, key)) = pq.pop() {
        dbg!(node, key);
        assert!(!removed.contains(&node));
        assert_eq!(vec[node], key);
    }
    assert!(pq.is_empty());
}
