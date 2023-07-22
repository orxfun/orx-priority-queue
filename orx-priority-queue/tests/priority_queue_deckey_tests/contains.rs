use orx_priority_queue::PriorityQueueDecKey;

pub fn test_contains<P>(mut pq: P)
where
    P: PriorityQueueDecKey<usize, f64>,
{
    pq.clear();
    assert!(pq.is_empty());

    for i in 0..30 {
        if i % 7 != 0 {
            pq.push(i, 0f64);
        }
    }

    for i in 0..30 {
        assert_eq!(i % 7 != 0, pq.contains(&i));
    }
}
