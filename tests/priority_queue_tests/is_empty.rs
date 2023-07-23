use orx_priority_queue::PriorityQueue;

pub fn test_is_empty<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    pq.clear();
    assert!(pq.is_empty());

    pq.push(0, 0f64);
    assert!(!pq.is_empty());

    pq.pop();
    assert!(pq.is_empty());
}
