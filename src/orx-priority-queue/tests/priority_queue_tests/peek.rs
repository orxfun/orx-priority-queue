use orx_priority_queue::{NodeKeyRef, PriorityQueue};

pub fn test_peek<P>(mut pq: P)
where
    P: PriorityQueue<usize, f64>,
{
    pq.clear();
    assert!(pq.peek().is_none());

    pq.push(1, 2.0);
    assert_eq!(Some(&1), pq.peek().map(|nk| nk.node()));
    assert_eq!(Some(&2.0), pq.peek().map(|nk| nk.key()));

    pq.push(2, 3.0);
    assert_eq!(Some(&1), pq.peek().map(|nk| nk.node()));
    assert_eq!(Some(&2.0), pq.peek().map(|nk| nk.key()));

    pq.push(3, 1.0);
    assert_eq!(Some(&3), pq.peek().map(|nk| nk.node()));
    assert_eq!(Some(&1.0), pq.peek().map(|nk| nk.key()));

    pq.pop();
    assert_eq!(Some(&1), pq.peek().map(|nk| nk.node()));
    assert_eq!(Some(&2.0), pq.peek().map(|nk| nk.key()));

    pq.pop();
    assert_eq!(Some(&2), pq.peek().map(|nk| nk.node()));
    assert_eq!(Some(&3.0), pq.peek().map(|nk| nk.key()));

    pq.pop();
    assert!(pq.peek().is_none());
}
