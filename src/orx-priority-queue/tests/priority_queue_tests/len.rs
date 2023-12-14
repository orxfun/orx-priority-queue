use orx_priority_queue::PriorityQueue;

pub fn test_len<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    pq.clear();
    assert_eq!(0, pq.len());

    for i in 0..10 {
        pq.push(i, 0f64);
        assert_eq!(i + 1, pq.len());
    }

    for i in 0..6 {
        pq.pop();
        assert_eq!(10 - i - 1, pq.len());
    }

    assert_eq!(4, pq.len());

    pq.clear();
    assert_eq!(0, pq.len());
}
