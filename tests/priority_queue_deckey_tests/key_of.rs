use orx_priority_queue::PriorityQueueDecKey;
use rand::prelude::*;

pub fn test_key_of<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    pq.clear();
    assert!(pq.is_empty());
    assert_eq!(None, pq.key_of(&0));

    let mut rng = rand::rng();
    let mut vec = Vec::new();

    for node in 0..100 {
        let priority = rng.random();
        pq.push(node, priority);
        vec.push(priority);
    }
    assert_eq!(100, pq.len());
    assert_eq!(100, vec.len());

    for (i, key) in vec.iter().enumerate() {
        assert_eq!(Some(*key), pq.key_of(&i));
    }
    assert_eq!(None, pq.key_of(&100));
}
