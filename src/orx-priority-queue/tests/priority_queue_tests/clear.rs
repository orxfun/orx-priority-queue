use orx_priority_queue::PriorityQueue;

pub fn test_clear<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    pq.clear();
    assert!(pq.is_empty());

    for i in 0..10 {
        pq.push(i, 0f64);
        assert_eq!(i + 1, pq.len());
    }

    // clear with items
    pq.clear();
    assert_eq!(0, pq.len());

    // clear while empty
    pq.clear();
    assert!(pq.is_empty());
}
